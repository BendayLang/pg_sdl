#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use itertools::Itertools;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::{Canvas};
use sdl2::video::Window;

use app::App;
use input::Input;
use crate::blocs::{Bloc, draw_bloc, set_child};
use crate::canvas::{fill_rect};
use crate::color::{darker, paler, Colors, hsv_color};
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
	sliders: Vec<Slider>,
	id_counter: u32,
	bloc_state: AppState,
	blocs: HashMap<u32, Bloc>,
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
				None,
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
				rect!(500, 150, 150, 18),
				Some(4),
				SliderType::Discrete {
					snap: 6,
					default_value: 3,
					display: Some(Box::new(|value| format!("R{}", 2_u32.pow(value + 1)))),
				},
			),
			Slider::new(
				Colors::ORANGE,
				rect!(700, 80, 40, 150),
				Some(20),
				SliderType::Continuous {
					default_value: 0.25,
					display: Some(Box::new(|value| format!("{:.2}", value * 100.0 - 50.0))),
				},
			)],
		id_counter: 0,
		bloc_state: AppState::Idle,
		blocs: HashMap::new(),
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
					let maybe_parent_bloc: Option<&Bloc> =
						app.blocs.values().into_iter().find(|bloc| moving_bloc.collide_bloc(bloc));
					
					let collide_with: Option<u32> = {
						let mut temp = None;
						for (id, bloc) in &app.blocs {
							if id == &moving_bloc_id { continue; }
							if moving_bloc.collide_bloc(bloc) { temp = Some(id); }
						}
						temp
					}.copied();
					
					if let Some(parent_id) = collide_with {
						set_child(moving_bloc_id, parent_id, &mut app.blocs);
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
		
		let radius = 2_u32.pow(app.sliders[0].get_value() as u32 + 1);
		DrawRenderer::rounded_box(canvas, 100, 300, 420, 500, radius as i16, Colors::GREEN).expect("DrawRenderer failed");
		DrawRenderer::rounded_rectangle(canvas, 100, 300, 420, 500, radius as i16, Colors::BLACK).expect("DrawRenderer failed");
		
		for (_id, bloc) in &app.blocs {
			if bloc.parent != None { continue; }
			draw_bloc(bloc, &app.blocs, canvas, text_drawer);
		}
		
		// text of blocs
		let text = format!("{}", app.blocs.iter()
		                            .map(|(id, bloc)| format!(" {}: {} ", id, bloc)).join("\n"));
		text_drawer.draw(canvas,
		                 &Text::new(text, 12.0),
		                 point!(130.0, 550.0),
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
