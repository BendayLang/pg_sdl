use crate::canvas::draw_rect;
use crate::prelude::*;
use crate::{
	color::{darker, Colors},
	input::{Input, KeyState},
	text::TextDrawer,
	widgets::Widget,
	widgets::{HOVER, PUSH},
};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// A button is a widget that it can be clicked.
pub struct Button {
	color: Color,
	hovered_color: Color,
	pushed_color: Color,
	rect: Rect,
	corner_radius: Option<u16>,
	text_style: TextStyle,
	text: String,
	hovered: bool,
	pub state: KeyState,
}

impl Button {
	pub fn new(color: Color, rect: Rect, corner_radius: Option<u16>, text_style: TextStyle, text: String) -> Self {
		Self {
			color,
			hovered_color: darker(color, HOVER),
			pushed_color: darker(color, PUSH),
			rect,
			corner_radius,
			text_style,
			text,
			hovered: false,
			state: KeyState::new(),
		}
	}
	pub fn set_text(&mut self, new_text: String) {
		self.text = new_text;
	}
}

impl Widget for Button {
	fn update(&mut self, input: &Input, _delta: f32, _text_drawer: &mut TextDrawer) -> bool {
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

	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		let color = if self.state.is_pressed() | self.state.is_down() {
			self.pushed_color
		} else if self.hovered {
			self.hovered_color
		} else {
			self.color
		};

		fill_rect(canvas, self.rect, color, self.corner_radius);
		draw_rect(canvas, self.rect, Colors::BLACK, self.corner_radius);

		text_drawer.draw(canvas, self.rect.center(), &self.text_style, &self.text, Align::Center);
	}
}
