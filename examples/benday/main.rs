// #![allow(dead_code, unused_variables)]
mod blocs;
use crate::blocs::{BlocContainer, BlocElement, Skeleton};
use blocs::{print::Print, Bloc};
use nalgebra::{Point2, Vector2};
use pg_sdl::app::{App, PgSdl};
use pg_sdl::camera::Camera;
use pg_sdl::color::{hsv_color, Colors};
use pg_sdl::input::Input;
use pg_sdl::rect;
use pg_sdl::text::{TextDrawer, TextStyle};
use pg_sdl::widgets::{Button, Widgets};
use sdl2::render::Canvas;
use sdl2::ttf::FontStyle;
use sdl2::video::Window;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Element {
	bloc_id: u32,
	bloc_element: BlocElement,
}

#[derive(PartialEq, Debug, Clone)]
struct Container {
	bloc_id: u32,
	bloc_container: BlocContainer,
}

enum AppState {
	Idle { selected_element: Option<Element>, hovered_element: Option<Element> },
	BlocMoving { moving_bloc_id: u32, delta: Vector2<f64>, hovered_container: Option<Container> },
}

pub struct MyApp {
	camera: Camera,
	id_counter: u32,
	app_state: AppState,
	blocs: HashMap<u32, Box<dyn Bloc>>,
	blocs_order: Vec<u32>,
}

impl App for MyApp {
	fn update(&mut self, _delta: f64, input: &Input, widgets: &mut Widgets) -> bool {
		let mut changed = false;

		match &self.app_state {
			AppState::Idle { selected_element, hovered_element } => {
				changed |= self.camera.update(input, selected_element.is_some());

				// Add new bloc
				if widgets.get_button("Add").state.is_pressed() {
					let id = self.id_counter;
					let new_bloc =
						Print::new(id, hsv_color((id * 15) as u16, 1.0, 1.0), Point2::new(8.0, 10.0) * id as f64);
					self.blocs.insert(id, Box::new(new_bloc));
					self.blocs_order.push(id);
					self.id_counter += 1;
				}
				// Mouse click
				else if input.mouse.left_button.is_pressed() {
					if let Some(Element { bloc_id, bloc_element }) = hovered_element {
						match bloc_element {
							// Select a bloc
							BlocElement::Body => {
								// Rearrange blocs order
								let mut new_blocs_order = self.blocs_order.clone();
								let childs =
									self.blocs.get(bloc_id).unwrap().get_skeleton().get_recursive_childs(&self.blocs);
								let childs_order_ids = childs
									.iter()
									.rev()
									.map(|child_id| {
										new_blocs_order
											.remove(new_blocs_order.iter().position(|i| i == child_id).unwrap())
									})
									.collect::<Vec<u32>>();
								new_blocs_order.extend(childs_order_ids);
								self.blocs_order = new_blocs_order;

								// Select a bloc
								if let Some(Container { bloc_id: parent_id, bloc_container }) =
									self.blocs.get(bloc_id).unwrap().get_skeleton().get_parent().clone()
								{
									match bloc_container {
										BlocContainer::Slot { slot_id } => {
											self.blocs
												.get_mut(&parent_id)
												.unwrap()
												.get_skeleton_mut()
												.set_slot_empty(slot_id);
										}
										BlocContainer::Sequence { .. } => (),
									}
									self.blocs.get_mut(bloc_id).unwrap().get_skeleton_mut().set_parent(None);
								}

								let delta = self.blocs.get(bloc_id).unwrap().get_skeleton().get_position()
									- self.camera.transform.inverse() * input.mouse.position.cast();
								self.app_state =
									AppState::BlocMoving { moving_bloc_id: *bloc_id, delta, hovered_container: None };
							}
							// Select a slot's textbox
							BlocElement::Slot(_) => {
								let element = Element { bloc_id: *bloc_id, bloc_element: *bloc_element };
								self.app_state =
									AppState::Idle { selected_element: Some(element), hovered_element: Some(element) };
							}
							_ => (),
						}
					}
					// Click in void
					else {
						self.app_state = AppState::Idle { selected_element: None, hovered_element: None };
					}
					changed = true;
				}
				// Update the (mouse) hovered element
				else if !input.mouse.delta.is_empty() {
					let mouse_position = self.camera.transform.inverse() * input.mouse.position.cast();
					let mut new_hovered_element = None;
					for id in self.blocs_order.iter().rev() {
						if let Some(bloc_element) =
							self.blocs.get(&id).unwrap().get_skeleton().collide_element(mouse_position)
						{
							new_hovered_element = Some(Element { bloc_id: *id, bloc_element });
							break;
						}
					}
					if &new_hovered_element != hovered_element {
						self.app_state = AppState::Idle {
							selected_element: *selected_element,
							hovered_element: new_hovered_element,
						};
						changed = true;
					}
				}
			}

			AppState::BlocMoving { moving_bloc_id, delta, hovered_container } => {
				// Release the bloc
				if input.mouse.left_button.is_released() {
					if let Some(Container { bloc_id, bloc_container }) = hovered_container {
						match bloc_container {
							BlocContainer::Slot { slot_id } => {
								self.blocs
									.get_mut(bloc_id)
									.unwrap()
									.get_skeleton_mut()
									.set_slot_child(*slot_id, *moving_bloc_id);
								self.blocs
									.get_mut(moving_bloc_id)
									.unwrap()
									.get_skeleton_mut()
									.set_parent(hovered_container.clone());
								// Update layout and childs positions
								let childs =
									self.blocs.get(bloc_id).unwrap().get_skeleton().get_recursive_childs(&self.blocs);
								childs.iter().for_each(|child_id| update_layout(*child_id, &mut self.blocs));
								childs
									.iter()
									.rev()
									.for_each(|child_id| update_childs_positions(*child_id, &mut self.blocs));
							}
							BlocContainer::Sequence { sequence_id, place } => {
								// TODO release bloc in sequence
							}
						}
					}

					let element = Element { bloc_id: *moving_bloc_id, bloc_element: BlocElement::Body };
					self.app_state = AppState::Idle { selected_element: Some(element), hovered_element: Some(element) };
					changed = true;
				// Move the bloc
				} else if !input.mouse.delta.is_empty() {
					let mouse_position = self.camera.transform.inverse() * input.mouse.position.cast();
					self.blocs
						.get_mut(&moving_bloc_id)
						.unwrap()
						.get_skeleton_mut()
						.set_position(mouse_position + delta);

					// Update the (moving bloc) hovered container
					let moving_bloc = self.blocs.get(&moving_bloc_id).unwrap().get_skeleton();
					let (mut new_hovered_container, mut ratio) = (None, 0.0);
					self.blocs_order.iter().for_each(|bloc_id| {
						if bloc_id != moving_bloc_id {
							if let Some((new_bloc_container, new_ratio)) = self
								.blocs
								.get(&bloc_id)
								.unwrap()
								.get_skeleton()
								.collide_container(*moving_bloc.get_position(), *moving_bloc.get_size())
							{
								if new_ratio > ratio {
									new_hovered_container =
										Some(Container { bloc_id: *bloc_id, bloc_container: new_bloc_container });
									ratio = new_ratio;
								}
							}
						}
					});
					if &new_hovered_container != hovered_container {
						self.app_state = AppState::BlocMoving {
							moving_bloc_id: *moving_bloc_id,
							delta: *delta,
							hovered_container: new_hovered_container,
						};
					}
					changed = true;
				}
			}
		}
		changed
	}

	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		self.camera.draw_grid(canvas, text_drawer, Colors::LIGHT_GREY, true, false);

		self.blocs_order.iter().for_each(|bloc_id| {
			let (moving, selected, hovered) = match &self.app_state {
				AppState::Idle { selected_element, hovered_element } => (
					false,
					if let Some(Element { bloc_id: element_bloc_id, bloc_element }) = selected_element {
						if bloc_id == element_bloc_id {
							Some(bloc_element)
						} else {
							None
						}
					} else {
						None
					},
					if let Some(Element { bloc_id: element_bloc_id, bloc_element }) = hovered_element {
						if bloc_id == element_bloc_id {
							Some(bloc_element)
						} else {
							None
						}
					} else {
						None
					},
				),
				AppState::BlocMoving { moving_bloc_id: selected_id, .. } => {
					if bloc_id == selected_id {
						(true, Some(&BlocElement::Body), Some(&BlocElement::Body))
					} else {
						(false, None, None)
					}
				}
			};
			self.blocs.get(bloc_id).unwrap().get_skeleton().draw(
				canvas,
				text_drawer,
				&self.camera,
				moving,
				selected,
				hovered,
			);
		});

		// text of blocs
		/*
		if !self.blocs.is_empty() {
			let text = format!(
				"{}",
				self.blocs
					.iter()
					.map(|(id, bloc)| format!(" {}: {} ", id, bloc.repr(&self.blocs)))
					.collect::<Vec<String>>()
					.join("  /  ")
			);
			text_drawer.draw(canvas, Point::new(100, 60), &TextStyle::default(), &text, Align::Left);
		}
		 */
	}
}

fn main() {
	let resolution = Vector2::new(1280, 720);
	let camera = Camera::new(resolution, 6, 3.0, 5.0, -4000.0, 4000.0, -5000.0, 5000.0);

	let my_app = &mut MyApp {
		camera,
		id_counter: 0,
		app_state: AppState::Idle { selected_element: None, hovered_element: None },
		blocs: HashMap::new(),
		blocs_order: Vec::new(),
	};

	let mut app = PgSdl::init("Benday", resolution.x, resolution.y, Some(60), true, Colors::LIGHT_GREY);

	app.add_widget(
		"Add",
		Box::new(Button::new(
			Colors::ROYAL_BLUE,
			rect!(100, 100, 200, 100),
			Some(9),
			TextStyle::new(20, None, Colors::BLACK, FontStyle::NORMAL),
			"New bloc".to_string(),
		)),
	);

	app.run(my_app);
}

fn update_layout(bloc_id: u32, blocs: &mut HashMap<u32, Box<dyn Bloc>>) {
	let mut bloc = blocs.remove(&bloc_id).unwrap();
	bloc.get_skeleton_mut().update_layout(&blocs);
	blocs.insert(bloc_id, bloc);
}

fn update_childs_positions(bloc_id: u32, blocs: &mut HashMap<u32, Box<dyn Bloc>>) {
	let mut bloc = blocs.remove(&bloc_id).unwrap();
	bloc.get_skeleton_mut().update_child_position(blocs);
	blocs.insert(bloc_id, bloc);
}

/*
// Update layout of blocs
self.blocs.iter().for_each(|(id, bloc)| {
	if bloc.get_skeleton().is_root {
		let recursive_childs = bloc.get_skeleton().get_recursive_childs(&self.blocs);
		recursive_childs.iter().for_each(|child_id| {
			self.blocs.get_mut(child_id).unwrap().get_skeleton_mut().update_size_and_position(&self.blocs);
		});
	}
});
*/
