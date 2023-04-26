use fontdue::layout::{HorizontalAlign, VerticalAlign};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::input::KeyState;
use crate::{Colors, darker, fill_rect, Input, Text, TextDrawer, Widget};
use crate::canvas::draw_rounded_rect;
use crate::widgets::{HOVER, PUSH};

/// A button is a widget that it can be clicked.
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
	fn update(&mut self, input: &Input, _delta: f32) -> bool {
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
			draw_rounded_rect(canvas, self.rect, radius);
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