#![allow(dead_code, unused_variables)]
mod blocs;
use crate::blocs::Bloc;
use crate::blocs::Skeleton;
use nalgebra::{Point2, Vector2};
use pg_sdl::prelude::*;
use sdl2::ttf::FontStyle;
use std::collections::HashMap;

enum AppState {
	Idle,
	Selected { id: u32, delta: Vector2<f64> },
}

pub struct MyApp {
	camera: Camera,
	id_counter: u32,
	app_state: AppState,
	blocs: HashMap<u32, Skeleton>,
	blocs_bis: HashMap<u32, Box<dyn Bloc>>,
	blocs_order: Vec<u32>,
}

impl App for MyApp {
	fn update(&mut self, _delta: f64, input: &Input, widgets: &mut Widgets) -> bool {
		let mut changed = false;
		match self.app_state {
			AppState::Idle => {
				changed |= self.camera.update(input);

				// Add new bloc
				if widgets.get_button("Add").state.is_pressed() {
					let id = self.id_counter;
					self.id_counter += 1;
					self.blocs.insert(
						id,
						Skeleton::new_variable_assignment(
							hsv_color((id * 30) as u16, 1.0, 1.0),
							Point2::new(8.0, 10.0) * id as f64,
							&self.blocs,
						),
					);
					self.blocs_order.push(id);
				}
				// Select a bloc
				else if input.mouse.left_button.is_pressed() {
					let mouse_position = self.camera.transform.inverse() * input.mouse.position.cast();
					for id in self.blocs_order.iter().rev() {
						let bloc = self.blocs.get(&id).unwrap();
						if bloc.collide(mouse_position.coords) {
							self.app_state =
								AppState::Selected { id: id.clone(), delta: bloc.position - mouse_position };
							changed = true;
							break;
						}
					}
					if let AppState::Selected { id, .. } = self.app_state {
						// Reorder blocs order
						let temp = self.blocs_order.iter().position(|i| i == &id).unwrap();
						let temp = self.blocs_order.remove(temp);
						self.blocs_order.push(temp);
					};
				}
			}
			AppState::Selected { id: moving_bloc_id, delta } => {
				// Move a bloc
				self.blocs.get_mut(&moving_bloc_id).unwrap().position =
					self.camera.transform.inverse() * input.mouse.position.cast() + delta;
				// .move_by(input.mouse.delta);
				changed |= !input.mouse.delta.is_empty();

				if input.mouse.left_button.is_released() {
					let moving_bloc = self.blocs.get(&moving_bloc_id).unwrap();
					let maybe_parent_bloc: Option<&Skeleton> =
						self.blocs.values().into_iter().find(|bloc| moving_bloc.collide_bloc(&bloc));

					let collide_with: Option<u32> = {
						let mut temp = None;
						for (id, bloc) in &self.blocs {
							if id == &moving_bloc_id {
								continue;
							}
							if moving_bloc.collide_bloc(bloc) {
								temp = Some(id);
							}
						}
						temp
					}
					.copied();

					if let Some(parent_id) = collide_with {
						// set_child(moving_bloc_id, parent_id, &mut self.blocs);
					}

					changed = true;
					self.app_state = AppState::Idle;
				}
			}
		}
		changed
	}

	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		self.camera.draw_grid(canvas, text_drawer, Colors::LIGHT_GREY, true, false);
		/*
		// old way of rendering blocs
		let texture_creator = canvas.texture_creator();
		for (_id, bloc) in &self.blocs {
			if bloc.parent != None {
				continue;
			}
			let surface = draw_bloc(bloc, &self.blocs, canvas, text_drawer, &texture_creator);
			let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string()).unwrap();
			canvas.copy(&texture, None, Some(bloc.get_rect())).unwrap();
		}
		 */

		self.blocs_order.iter().for_each(|id| {
			let selected = match self.app_state {
				AppState::Selected { id: selected_id, .. } => id == &selected_id,
				_ => false,
			};
			let bloc = self.blocs.get(id).unwrap();
			bloc.draw(canvas, text_drawer, &self.camera, &self.blocs, selected);
		});

		// text of blocs
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
	}
}

fn main() {
	let resolution = Vector2::new(1280, 720);
	let camera = Camera::new(resolution, 6, 3.0, 5.0, -4000.0, 4000.0, -5000.0, 5000.0);
	let my_app =
		&mut MyApp { camera, id_counter: 0, app_state: AppState::Idle, blocs: HashMap::new(), blocs_order: Vec::new() };

	let mut app = PgSdl::init("Benday", resolution.x, resolution.y, Some(60), true, Colors::LIGHT_GREY);

	app.add_widget(
		"Add",
		Box::new(Button::new(
			Colors::ROYAL_BLUE,
			rect!(100, 100, 200, 100),
			Some(9),
			TextStyle::new(20, None, Color::BLACK, FontStyle::NORMAL),
			"New bloc".to_string(),
		)),
	);

	app.run(my_app);
}
