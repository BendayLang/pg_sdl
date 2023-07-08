use crate::blocs::containers::Slot;
use nalgebra::{Point2, Vector2};
use pg_sdl::camera::Camera;
use pg_sdl::color::paler;
use pg_sdl::text::TextDrawer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct TextBox {
	default_text: String,
	default_color: Color,
	text: String,
	color: Color,
	size: Vector2<f64>,
}

impl TextBox {
	pub fn new(size: Vector2<f64>, color: Color, default_text: String) -> Self {
		Self { default_text, default_color: paler(color, 0.2), text: String::new(), color: paler(color, 0.5), size }
	}
	pub fn get_size(&self) -> Vector2<f64> {
		self.size
	}
	pub fn get_text(&self) -> String {
		self.text.clone()
	}
	pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera, position: Vector2<f64>) {
		camera.fill_rounded_rect(canvas, self.color, Point2::from(position), self.size, Slot::RADIUS);
	}
}
