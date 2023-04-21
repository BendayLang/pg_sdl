#![allow(dead_code, unused_imports, unused_variables)]

use std::borrow::BorrowMut;
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;

mod app;
mod canvas;
mod draw_circle;
mod input;
mod text;
mod utils;
mod widgets;
mod color;

use app::App;
pub use input::Input;
use crate::canvas::{fill_rect};
use crate::color::{hsv_color, darker, paler, Colors};
use crate::draw_circle::{draw_circle, fill_circle};
use crate::widgets::{Button, Orientation, Slider};
use crate::text::{Text, TextDrawer};
use crate::widgets::Widget;


pub struct MyApp {
	button1: Button,
	button2: Button,
	slider1: Slider<i32>,
	slider2: Slider<f32>,
	text: String,
}

fn main() {
	let my_app = &mut MyApp {
		button1: Button::new(
			Colors::ROYAL_BLUE,
			rect!(100, 100,200,100),
			Some(7),
			Some(Text { text: "Réponse à Loïc".to_string(), ..Default::default() }),
		),
		button2: Button::new(
			Colors::GREY,
			rect!(550, 20, 80, 50),
			Some(7),
			None,
		),
		slider1: Slider::new(
			Colors::GREEN,
			rect!(500, 150, 180, 18),
			Some(4),
			[-10, 8],
			Some(2),
			0,
		),
		slider2: Slider::new(
			Colors::ORANGE,
			rect!(700, 80, 30, 150),
			Some(14),
			[0.0, 2.0],
			None,
			1.0,
		),
		text: String::new(),
	};
	
	fn update(app: &mut MyApp, delta: f32, input: &Input) -> bool {
		let mut changed = false;
		changed |= app.button1.update(&input, delta);
		changed |= app.button2.update(&input, delta);
		changed |= app.slider1.update(&input, delta);
		changed |= app.slider2.update(&input, delta);
		
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
		app.button1.draw(canvas, text_drawer);
		app.button2.draw(canvas, text_drawer);
		app.slider1.draw(canvas, text_drawer);
		app.slider2.draw(canvas, text_drawer);
		
		let text = app.text.clone();
		text_drawer.draw(canvas,
		                 &Text { text, color: Colors::BLUE, ..Default::default() },
		                 point!(130.0, 250.0),
		                 None,
		                 None,
		                 HorizontalAlign::Left,
		                 VerticalAlign::Top);
		
		fill_rect(canvas, rect!(400, 250, 80, 120), Some(90));
		fill_rect(canvas, rect!(600, 310, 120, 80), Some(90));
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
