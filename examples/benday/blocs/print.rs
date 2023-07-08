use crate::blocs::{Bloc, Skeleton};
use nalgebra::Vector2;
use pg_sdl::camera::Camera;
use pg_sdl::color::hsv_color;
use pg_sdl::prelude::TextDrawer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;

pub struct Print {
	skeleton: Skeleton,
}
impl Print {
	const COLOR: Color = hsv_color(330, 0.3, 1.0);
}

impl Bloc for Print {
	fn new()
	
	fn get_size(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64> {
		self.skeleton.slots.get(0).unwrap().get_size(blocs) + Vector2::new(2.0, 2.0) * Skeleton::MARGIN
	}

	fn slot_position(&self, slot_id: u16) -> Vector2<f64> {
		todo!()
	}

	fn sequence_position(&self, sequence_id: u16) -> Vector2<f64> {
		todo!()
	}

	fn button_size(&self, button_id: u16) -> Vector2<f64> {
		todo!()
	}

	fn button_position(&self, button_id: u16) -> Vector2<f64> {
		todo!()
	}

	fn draw_button(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera) {
		todo!()
	}

	fn button_function(&mut self, button_id: u16) -> bool {
		todo!()
	}
}
