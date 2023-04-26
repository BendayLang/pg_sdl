#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use itertools::Itertools;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas};
use sdl2::video::Window;

use app::App;
use input::Input;
use crate::blocs::Bloc;
use crate::canvas::{fill_rect};
use crate::color::{darker, paler, Colors, hsv_color};
use crate::draw_circle::{draw_circle};
use crate::text::{Text, TextDrawer};
use crate::widgets::button::Button;
use crate::widgets::slider::{Slider, SliderType};
use crate::widgets::Widget;


mod app;
mod canvas;
mod draw_circle;
mod input;
mod text;
mod utils;
mod widgets;
mod color;
mod camera;
mod blocs;


enum AppState {
	Idle,
	Selected { id: u32 },
}


pub struct MyApp {
	buttons: Vec<Button>,
	sliders: Vec<Slider<i32>>,
	id_counter: u32,
	bloc_state: AppState,
	blocs: HashMap<u32, Bloc>,
	text: String,
}


fn main() {
	let my_app = &mut MyApp {
		buttons: vec![
			Button::new(
				Colors::ROYAL_BLUE,
				rect!(100, 100, 200, 100),
				Some(9),
				Some(Text::new("New bloc".to_string(), 20.0))),
			Button::new(
				Colors::GREY,
				rect!(550, 20, 80, 50),
				Some(7),
				None,
			),
			Button::new(
				Colors::GREEN,
				rect!(400, 200, 100, 100),
				Some(8),
				Some(Text::new("Reset Slider 1".to_string(), 20.0))),
		],
		sliders: vec![
			Slider::new(
				Colors::GREEN,
				rect!(500, 150, 180, 18),
				Some(4),
				SliderType::Discrete {
					snap: 10,
					default_value: 5,
					value_getter_function: Box::new(|value| Box::new((value * 10) as i32))
				},
				true,
			),
			Slider::new(
				Colors::ORANGE,
				rect!(700, 80, 40, 150),
				Some(20),
				SliderType::Continuous {
					default_value: 0.25,
					value_getter_function: Box::new(|value| Box::new((value * 100.0 - 50.0).round() as i32))
				},
				true,
			)],
		id_counter: 0,
		bloc_state: AppState::Idle,
		blocs: HashMap::new(),
		text: String::new(),
	};
	
	fn update(app: &mut MyApp, delta: f32, input: &Input) -> bool {
		let mut changed = false;
		
		let widgets: Vec<&mut dyn Widget> = app.buttons.iter_mut()
		                                       .map(|button| button as &mut dyn Widget)
		                                       .chain(app.sliders.iter_mut()
		                                                 .map(|slider| slider as &mut dyn Widget))
		                                       .collect();
		for widget in widgets {
			changed |= widget.update(&input, delta);
		}
		
		if app.buttons[2].state.is_pressed() {
			app.sliders[0].reset_value();
		}
		
		match app.bloc_state {
			AppState::Idle => {
				// Add a bloc
				if app.buttons[0].state.is_pressed() {
					let id = app.id_counter;
					app.id_counter += 1;
					app.blocs.insert(id, Bloc::new(
						hsv_color((id * 30) as u16, 1.0, 1.0),
						rect!(10 * id + 120, 10 * id + 230, 110, 80),
					));
				}
				// Select a bloc
				if input.mouse.left_button.is_pressed() {
					let mouse_pos = input.mouse.position;
					for (id, bloc) in &mut app.blocs {
						if bloc.collide(mouse_pos) {
							app.bloc_state = AppState::Selected { id: *id };
							changed = true;
						}
					}
				}
			},
			AppState::Selected { id: moving_bloc_id } => {
				// Move a bloc
				app.blocs.get_mut(&moving_bloc_id).unwrap().move_by(input.mouse.delta);
				changed |= input.mouse.delta != point!(0, 0);
				
				if input.mouse.left_button.is_released() {
					let moving_bloc = app.blocs.get(&moving_bloc_id).unwrap();
					let mut id_bloc: Option<(&u32, &mut Bloc)> =
						app.blocs.iter_mut().find(|(_id, bloc)| moving_bloc.collide_bloc(bloc));
					
					// id_bloc.unwrap().1.set_child(moving_bloc_id);
					if let Some((id, parent_bloc)) = id_bloc {
						// let parent_bloc = app.blocs.get_mut(&id).unwrap();
						parent_bloc.set_child(moving_bloc_id);
					}
					
					changed = true;
					app.bloc_state = AppState::Idle;
				}
			},
		}
		
		changed
	}
	
	fn draw(app: &mut MyApp, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
		let widgets = app.buttons.iter_mut()
		                 .map(|button| button as &mut dyn Widget)
		                 .chain(app.sliders.iter_mut()
		                           .map(|slider| slider as &mut dyn Widget))
		                 .collect::<Vec<&mut dyn Widget>>();
		
		for widget in widgets {
			widget.draw(canvas, text_drawer);
		}
		
		canvas.set_draw_color(Colors::VIOLET);
		draw_circle(canvas, point!(500, 400), 100, 20);
		
		canvas.set_draw_color(Colors::RED_ORANGE);
		let width: u32 = 20;
		let rect = rect!(650, 350, 150, 100);
		let rects = (0..width).map(|i| rect!(rect.x as u32 + i, rect.y as u32 + i,
			rect.width() - 2 * i, rect.height() - 2 * i))
		                      .collect::<Vec<Rect>>();
		canvas.draw_rects(&rects).unwrap();
		
		for (_id, bloc) in &app.blocs {
			bloc.draw(canvas, text_drawer);
		}
		
		let text = app.text.clone();
		text_drawer.draw(canvas,
		                 &Text::new(text, 20.0),
		                 point!(130.0, 250.0),
		                 None,
		                 None,
		                 HorizontalAlign::Left,
		                 VerticalAlign::Top);
	}
	
	let mut app: App = App::init(
		"Benday",
		1200,
		720,
		Some(60.0),
		true,
		Colors::SKY_BLUE,
		update,
		draw,
	);
	
	app.run(my_app);
}
