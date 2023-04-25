use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::canvas::{draw_rect, draw_rounded_rect, fill_rect};
use crate::{App, Input, paler, point, rect};
use crate::color::{Colors, darker, hsv_color};
use crate::input::KeyState;
use crate::text::TextDrawer;
use crate::text::Text;
use num::NumCast;

const HOVER: f32 = 0.94;
const PUSH: f32 = 0.80;

/// A widget is a UI object that can be interacted with to take inputs from the user.
pub trait Widget {
	/// Update the widget based on the inputs
	fn update(&mut self, input: &Input, delta: f32) -> bool;
	/// Draw the widget on the canvas
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer);
}

/// A button is a simple widget, it can just be clicked.
pub struct Button {
	color: Color,
	hovered_color: Color,
	pushed_color: Color,
	rect: Rect,
	corner_radius: Option<u32>,
	text: Option<Text>,
	hovered: bool,
	pub state: KeyState,
}

impl Button {
	pub fn new(color: Color, rect: Rect, corner_radius: Option<u32>,
	           text: Option<Text>) -> Self {
		Self {
			color,
			hovered_color: darker(color, HOVER),
			pushed_color: darker(color, PUSH),
			rect,
			corner_radius,
			text,
			hovered: false,
			state: KeyState::new(),
		}
	}
}

impl Widget for Button {
	fn update(&mut self, input: &Input, delta: f32) -> bool {
		let mut changed = false;
		self.state.update();
		
		let hovered = self.rect.contains_point(input.mouse.position);
		if hovered != self.hovered {
			self.hovered = hovered;
			changed = true;
		}
		
		if input.mouse.left_button.is_pressed() && self.hovered {
			self.state.press();
			changed = true;
		} else if self.state.is_down() && input.mouse.left_button.is_released() {
			self.state.release();
			changed = true;
		}
		
		changed
	}
	
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
		canvas.set_draw_color(if self.state.is_pressed() | self.state.is_down() {
			self.pushed_color
		} else if self.hovered {
			self.hovered_color
		} else {
			self.color
		});
		fill_rect(canvas, self.rect, self.corner_radius);
		canvas.set_draw_color(Colors::BLACK);
		if let Some(radius) = self.corner_radius {
			//draw_rounded_rect(canvas, self.rect, radius);
		}
		
		if let Some(text) = &self.text {
			text_drawer.draw(canvas,
			                 text,
			                 self.rect.top_left(),
			                 Some(self.rect.width() as f32),
			                 Some(self.rect.height() as f32),
			                 HorizontalAlign::Center,
			                 VerticalAlign::Middle);
		}
	}
}

// #[derive(PartialEq)]
pub enum Orientation { Horizontal, Vertical }

pub enum SliderType { Discret { snap: u8, default_value: u8 }, Continuous { default_value: f32 } }

/// A slider is a widget that can be dragged to change a value.
pub struct Slider<R: ?Sized> {
	color: Color,
	hovered_color: Color,
	back_color: Color,
	hovered_back_color: Color,
	thumb_color: Color,
	hovered_thumb_color: Color,
	pushed_thumb_color: Color,
	rect: Rect,
	orientation: Orientation,
	corner_radius: Option<u32>,
	hovered: bool,
	pub state: KeyState,
	/// (0.0 - 1.0)
	value: f32,
	slider_type: SliderType,
	value_getter_function: Box<dyn Fn(f32) -> R>,
	draw_value: Option<Box<dyn Fn(&R) -> String>>,
}

impl<R> Slider<R> {
	pub fn new(color: Color, rect: Rect, corner_radius: Option<u32>,
	           slider_type: SliderType, value_getter_function: Box<dyn Fn(f32) -> R>,
	           draw_value: Option<Box<dyn Fn(&R) -> String>>) -> Self {
		let orientation = {
			if rect.width() > rect.height() { Orientation::Horizontal } else { Orientation::Vertical }
		};
		let thumb_color = Colors::LIGHT_GREY;
		let back_color = darker(paler(color, 0.5), 0.9);
		Self {
			color,
			hovered_color: darker(color, HOVER),
			back_color,
			hovered_back_color: darker(back_color, HOVER),
			thumb_color,
			hovered_thumb_color: darker(thumb_color, HOVER),
			pushed_thumb_color: darker(thumb_color, PUSH),
			rect,
			orientation,
			corner_radius,
			hovered: false,
			state: KeyState::new(),
			value: match slider_type {
				SliderType::Discret { snap, default_value } => default_value as f32 / snap as f32,
				SliderType::Continuous { default_value } => default_value,
			},
			slider_type,
			value_getter_function,
			draw_value,
		}
	}
	
	pub fn get_value(&self) -> R { (self.value_getter_function)(self.value) }
	
	fn thumb_position(&self) -> u32 { (self.value * self.length() as f32) as u32 }
	
	fn length(&self) -> u32 {
		match self.orientation {
			Orientation::Horizontal => self.rect.width() - self.rect.height(),
			Orientation::Vertical => self.rect.height() - self.rect.width(),
		}
	}
	
	fn thickness(&self) -> u32 {
		match self.orientation {
			Orientation::Horizontal => self.rect.height(),
			Orientation::Vertical => self.rect.width(),
		}
	}
}

impl<R> Widget for Slider<R> {
	fn update(&mut self, input: &Input, delta: f32) -> bool {
		let mut changed = false;
		self.state.update();
		
		let hovered = self.rect.contains_point(input.mouse.position);
		if hovered != self.hovered {
			self.hovered = hovered;
			changed = true;
		}
		
		if input.mouse.left_button.is_pressed() && self.hovered {
			self.state.press();
			changed = true;
		} else if self.state.is_down() && input.mouse.left_button.is_released() {
			self.state.release();
			changed = true;
		}
		
		if self.state.is_pressed() | self.state.is_down() {
			let value = {
				let point = input.mouse.position;
				let thumb_position = match self.orientation {
					Orientation::Horizontal => point.x() - self.rect.left(),
					Orientation::Vertical => self.rect.bottom() - point.y(),
				} - self.thickness() as i32 / 2;
				thumb_position.clamp(0, self.length() as i32) as f32 / self.length() as f32
			};
			
			let value = match self.slider_type {
				SliderType::Discret { snap, .. } => (value * snap as f32).round() / snap as f32,
				SliderType::Continuous { .. } => value,
			};
			
			if value != self.value {
				self.value = value;
				changed = true;
			}
		}
		
		changed
	}
	
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
		let b: f32 = 0.7;
		
		// Back bar
		let margin = (self.thickness() as f32 * (1.0 - b) / 2.0) as u32;
		
		let (back_rect, rect) = match self.orientation {
			Orientation::Horizontal => {
				(rect!(
					self.rect.left() + (self.thumb_position() + self.thickness() / 2) as i32,
					self.rect.top() + margin as i32,
					self.rect.width() - self.thumb_position() as u32 - self.thickness() / 2 -margin,
					self.rect.height() as f32 * b),
				 rect!(
					 self.rect.left() + margin as i32,
					 self.rect.top() + margin as i32,
					 self.thumb_position() + self.thickness() / 2 - margin,
					 self.rect.height() as f32 * b))
			}
			Orientation::Vertical => {
				(rect!(
					self.rect.left() + margin as i32,
					self.rect.top() + margin as i32,
					self.rect.width() as f32 * b,
					self.rect.height() - self.thumb_position() - self.thickness() / 2 - margin),
				 rect!(
					 self.rect.left() + margin as i32,
					 self.rect.bottom() - (self.thumb_position() + self.thickness() / 2) as i32,
					 self.rect.width() as f32 * b,
					 self.thumb_position() + self.thickness() / 2 -margin))
			}
		};
		
		canvas.set_draw_color(if self.hovered | self.state.is_pressed() | self.state.is_down() {
			self.hovered_back_color
		} else { self.back_color });
		fill_rect(canvas, back_rect, self.corner_radius);
		canvas.set_draw_color(if self.hovered | self.state.is_pressed() | self.state.is_down() {
			self.hovered_color
		} else { self.color });
		fill_rect(canvas, rect, self.corner_radius);
		
		// Pad
		canvas.set_draw_color(
			if self.state.is_pressed() | self.state.is_down() {
				self.pushed_thumb_color
			} else if self.hovered {
				self.hovered_thumb_color
			} else {
				self.thumb_color
			}
		);
		let rect = match self.orientation {
			Orientation::Horizontal => rect!(
				self.rect.left() + self.thumb_position() as i32, self.rect.top(),
				self.thickness(), self.thickness()),
			Orientation::Vertical => rect!(
				self.rect.left(), self.rect.bottom() - self.thumb_position() as i32 - self.thickness() as i32,
				self.thickness(), self.thickness())
		};
		fill_rect(canvas, rect, self.corner_radius);
		
		if let Some(draw_value) = &self.draw_value {
			let text = (*draw_value)(&self.get_value());
			text_drawer.draw(canvas,
			                 &Text::new(text, 20.0),
			                 rect.center(),
			                 None,
			                 None,
			                 HorizontalAlign::Left,
			                 VerticalAlign::Top,
			);
		}
	}
}
