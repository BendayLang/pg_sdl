use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::canvas::{fill_rect};
use crate::{App, Input, paler, point, rect};
use crate::color::{Colors, darker, hsv_color};
use crate::input::KeyState;
use crate::text::TextDrawer;
use crate::text::Text;
use num::NumCast;


pub trait Widget {
	fn update(&mut self, input: &Input, delta: f32) -> bool;
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer);
}

pub struct Button {
	color: Color,
	hovered_color: Color,
	pushed_color: Color,
	rect: Rect,
	corner_radius: Option<u32>,
	text: Option<Text>,
	pub state: KeyState,
	hovered: bool,
}

impl Button {
	pub fn new(color: Color, rect: Rect, corner_radius: Option<u32>, text: Option<Text>) -> Self {
		Self {
			color,
			hovered_color: darker(color, 0.92),
			pushed_color: darker(color, 0.7),
			rect,
			corner_radius,
			text,
			state: KeyState::new(),
			hovered: false,
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
		let color = if self.state.is_pressed() | self.state.is_down() {
			self.pushed_color
		} else if self.hovered {
			self.hovered_color
		} else {
			self.color
		};
		
		canvas.set_draw_color(color);
		fill_rect(canvas, self.rect, self.corner_radius);
		
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

pub enum Orientation { Horizontal, Vertical }
/*
struct T<T> where T: Copy + Sub + Sub<Output=T> + Add + Add<Output=T> + Mul + Mul<Output=T>
+ num::ToPrimitive + NumCast + Display + PartialEq + Default;
 */

pub struct Slider<T> {
	color: Color,
	back_color: Color,
	pad_color: Color,
	pad_hovered_color: Color,
	pad_pushed_color: Color,
	rect: Rect,
	orientation: Orientation,
	corner_radius: Option<u32>,
	pub state: KeyState,
	hovered: bool,
	span: [T; 2],
	snap: Option<T>,
	value: T,
}

impl<T> Slider<T> where T: Copy + Sub + Sub<Output = T> + Add + Add<Output = T> + Mul + Mul<Output = T>
+ num::ToPrimitive + NumCast + Display + PartialEq + Default {
	pub fn new(color: Color, rect: Rect, corner_radius: Option<u32>,
	           span: [T; 2], snap: Option<T>, value: T) -> Self {
		let orientation = {
			if rect.width() > rect.height() { Orientation::Horizontal } else { Orientation::Vertical }
		};
		let pad_color = Colors::LIGHT_GREY;
		Self {
			color,
			back_color: darker(paler(color, 0.5), 0.9),
			pad_color,
			pad_hovered_color: darker(pad_color, 0.92),
			pad_pushed_color: darker(pad_color, 0.7),
			rect,
			orientation,
			corner_radius,
			state: KeyState::new(),
			hovered: false,
			span,
			snap,
			value,
		}
	}
	
	fn pad_position(&self) -> i32 {
		let span0: f32 = NumCast::from(self.span[0]).unwrap_or_default();
		let span1: f32 = NumCast::from(self.span[1]).unwrap_or_default();
		let value: f32 = NumCast::from(self.value).unwrap_or_default();
		let length: f32 = (self.length() - self.width()) as f32;
		
		((value - span0) * length / (span1 - span0)) as i32
	}
	
	fn point_value(&self, point: Point) -> T {
		let pad_position: f32 = match self.orientation {
			Orientation::Horizontal => point.x as f32 - self.rect.left() as f32,
			Orientation::Vertical => point.y as f32 - self.rect.top() as f32,
		} - self.width() as f32 / 2.0;
		let length: f32 = (self.length() - self.width()) as f32;
		
		if pad_position / length <= 0.0 { return self.span[0]; } else if pad_position / length >= 1.0 { return self.span[1]; }
		
		let span0: f32 = NumCast::from(self.span[0]).unwrap_or_default();
		let span1: f32 = NumCast::from(self.span[1]).unwrap_or_default();
		
		let mut value: f32 = span0 + pad_position * (span1 - span0) / length;
		if let Some(snap) = self.snap {
			let snap: f32 = NumCast::from(snap).unwrap_or_default();
			value = (value / snap).round() * snap;
		}
		let value: T = NumCast::from(value).unwrap_or_default();
		value
	}
	
	fn length(&self) -> u32 {
		match self.orientation {
			Orientation::Horizontal => self.rect.width(),
			Orientation::Vertical => self.rect.height()
		}
	}
	
	fn width(&self) -> u32 {
		match self.orientation {
			Orientation::Horizontal => self.rect.height(),
			Orientation::Vertical => self.rect.width()
		}
	}
}

impl<T> Widget for Slider<T> where T: Copy + Add + Add<Output = T> + Sub + Sub<Output = T> + Mul +
Mul<Output = T> + num::ToPrimitive + NumCast + Display + PartialEq + Default {
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
		
		if self.state.is_down() {
			let value = self.point_value(input.mouse.position);
			if value != self.value {
				self.value = value;
				changed = true;
			}
		}
		
		changed
	}
	
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
		let b: f32 = 0.7;
		
		let (back_rect, rect) = match self.orientation {
			Orientation::Horizontal => {
				(rect!(
					self.rect.left() + self.pad_position() + self.width() as i32 / 2,
					self.rect.top() as f32 + self.width() as f32 * (1.0 - b) / 2.0,
					self.rect.width() - self.pad_position() as u32 - self.width() / 2,
					self.rect.height() as f32 * b),
				 rect!(
					 self.rect.left(),
					 self.rect.top() as f32 + self.width() as f32 * (1.0 - b) / 2.0,
					 self.pad_position() + self.width() as i32 / 2,
					 self.rect.height() as f32 * b))
			}
			Orientation::Vertical => {
				(rect!(
					self.rect.left() as f32 + self.width() as f32 * (1.0 - b) / 2.0,
					self.rect.top(),
					self.rect.width() as f32 * b,
					self.pad_position() + self.width() as i32 / 2),
				 rect!(
					 self.rect.left() as f32 + self.width() as f32 * (1.0 - b) / 2.0,
					 self.rect.top() + self.pad_position() + self.width() as i32 / 2,
					 self.rect.width() as f32 * b,
					 self.rect.height() - self.pad_position() as u32 - self.width() / 2))
			}
		};
		canvas.set_draw_color(self.back_color);
		fill_rect(canvas, back_rect, self.corner_radius);
		canvas.set_draw_color(self.color);
		fill_rect(canvas, rect, self.corner_radius);
		
		canvas.set_draw_color(
			if self.state.is_pressed() | self.state.is_down() {
				self.pad_pushed_color
			} else if self.hovered {
				self.pad_hovered_color
			} else {
				self.pad_color
			}
		);
		
		let rect = match self.orientation {
			Orientation::Horizontal => rect!(
				self.rect.left() + self.pad_position(), self.rect.top(), self.width(), self.width()),
			Orientation::Vertical => rect!(
				self.rect.left(), self.rect.top() + self.pad_position(), self.width(), self.width())
		};
		fill_rect(canvas, rect, self.corner_radius);
	}
}
