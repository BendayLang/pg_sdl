#![allow(dead_code, unused_imports, unused_variables)]

use std::borrow::BorrowMut;
use std::collections::HashMap;
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use itertools::Itertools;
use num::Num;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;

use app::App;
use input::Input;
use crate::blocs::Bloc;
use crate::canvas::{fill_rect};
use crate::color::{hsv_color, darker, paler, Colors};
use crate::draw_circle::{draw_circle, fill_circle};
use crate::widgets::{Button, Orientation, Slider};
use crate::text::{Text, TextDrawer};
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


pub struct MyApp {
	buttons: Vec<Button>,
	sliders: Vec<Slider>,
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
				Some(Text { text: "Réponse à Loïc".to_string(), ..Default::default() })),
			Button::new(
				Colors::GREY,
				rect!(550, 20, 80, 50),
				Some(7),
				None,
			)],
		sliders: vec![
			Slider::new(
				Colors::GREEN,
				rect!(500, 150, 180, 18),
				Some(4),
				[-10, 8],
				Some(2),
				0,
			),
			Slider::new(
				Colors::ORANGE,
				rect!(700, 80, 30, 150),
				Some(14),
				[0, 2],
				None,
				1,
			)],
		blocs: HashMap::from([(0, Bloc::new(
			Colors::MAGENTA, rect!(120, 230, 110, 80),
		))]),
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
		
		if app.buttons[0].state.is_pressed() {
			app.sliders[0].value = app.sliders[0].span[0];
		}
		
		if let Some(last_char) = input.last_char {
			app.text.push(last_char);
			changed = true;
		};
		if input.keys_state.backspace.is_pressed() {
			if let Some(_) = app.text.pop() { changed = true; }
		};
		
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
		
		for (id, bloc) in &app.blocs {
			bloc.draw(canvas, text_drawer);
		}
		
		let text = app.text.clone();
		text_drawer.draw(canvas,
		                 &Text { text, color: Colors::BLUE, ..Default::default() },
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
