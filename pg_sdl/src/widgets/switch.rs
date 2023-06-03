use crate::input::KeyState;
use crate::prelude::*;
use crate::widgets::{HOVER, PUSH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::FontStyle;

use sdl2::video::Window;

/// A switch is a widget that can be toggled __on__ or __off__
pub struct Switch {
	on_color: Color,
	hovered_on_color: Color,
	off_color: Color,
	hovered_off_color: Color,
	thumb_color: Color,
	hovered_thumb_color: Color,
	rect: Rect,
	orientation: Orientation,
	corner_radius: Option<u16>,
	hovered: bool,
	pub state: KeyState,
	switched: bool,
}

impl Switch {
	pub fn new(on_color: Color, off_color: Color, rect: Rect, corner_radius: Option<u16>) -> Self {
		let orientation = {
			if rect.width() > rect.height() {
				Orientation::Horizontal
			} else {
				Orientation::Vertical
			}
		};
		let thumb_color = Colors::LIGHT_GREY;
		Self {
			on_color,
			hovered_on_color: darker(on_color, HOVER),
			off_color,
			hovered_off_color: darker(off_color, HOVER),
			thumb_color,
			hovered_thumb_color: darker(thumb_color, HOVER),
			rect,
			orientation,
			corner_radius,
			hovered: false,
			state: KeyState::new(),
			switched: false,
		}
	}

	pub fn set_switched(&mut self, switched: bool) {
		self.switched = switched;
	}

	fn thumb_position(&self) -> u32 {
		self.switched as u32 * self.length()
	}

	fn length(&self) -> u32 {
		match self.orientation {
			Orientation::Horizontal => self.rect.width() - self.rect.height(),
			Orientation::Vertical => self.rect.height() - self.rect.width(),
		}
	}
}

impl Widget for Switch {
	fn update(&mut self, input: &Input, _delta: f32, _text_drawer: &mut TextDrawer) -> bool {
		let mut changed = false;
		self.state.update();

		let mouse_position = Point::new(input.mouse.position.x, input.mouse.position.y);
		let hovered = self.rect.contains_point(mouse_position);
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

		if self.state.is_pressed() {
			self.switched = !self.switched;
		}

		changed
	}

	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		let b: f32 = 0.7;

		let color = {
			if self.switched {
				if self.hovered {
					self.hovered_on_color
				} else {
					self.on_color
				}
			} else {
				if self.hovered {
					self.hovered_off_color
				} else {
					self.off_color
				}
			}
		};
		fill_rect(canvas, self.rect, color, self.corner_radius);

		let thickness = match self.orientation {
			Orientation::Horizontal => self.rect.height(),
			Orientation::Vertical => self.rect.width(),
		};
		let margin = (thickness as f32 * (1.0 - b) / 2.0) as u32;
		let dot_width = thickness - 2 * margin; // (thickness as f32 * b) as u32;

		// Pad
		let thumb_rect = match self.orientation {
			Orientation::Horizontal => {
				rect!(
					margin as i32 + self.rect.left() + self.thumb_position() as i32,
					margin as i32 + self.rect.top(),
					dot_width,
					dot_width
				)
			}
			Orientation::Vertical => rect!(
				margin as i32 + self.rect.left(),
				margin as i32 + self.rect.bottom() - self.thumb_position() as i32 - thickness as i32,
				dot_width,
				dot_width
			),
		};
		fill_rect(
			canvas,
			thumb_rect,
			if self.hovered { self.hovered_thumb_color } else { self.thumb_color },
			self.corner_radius.map(|r| (r as f32 * b) as u16),
		);
	}
}
