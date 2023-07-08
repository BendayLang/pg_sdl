use crate::blocs::{Bloc, Skeleton};
use nalgebra::{Point2, Vector2};
use pg_sdl::camera::Camera;
use pg_sdl::color::hsv_color;
use pg_sdl::prelude::TextDrawer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;

use super::containers::{HoveredOn, Slot};

pub struct While {
	skeleton: Skeleton,
}

impl While {
	const COLOR: Color = hsv_color(330, 0.3, 1.0);
	const TEXT_WIDTH: f64 = 0.0;

	fn new(position: Point2<f64>, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Self {
		let mut bloc = Self {
			skeleton: Skeleton {
				color: Self::COLOR,
				position,
				size: Vector2::zeros(),
				slots: vec![Slot::new(Self::COLOR, "value")],
				sequences: Vec::new(),
				hovered_on: HoveredOn::None,
			},
		};
		bloc.skeleton.size = bloc.get_size(blocs);
		bloc
	}
}

impl Bloc for While {
	fn get_size(&self, blocs: &HashMap<u32, Box<dyn Bloc>>) -> Vector2<f64> {
		// width = self.sequences[0].size.x + self.slots[0].size.x + self.text_width
		// height = max(self.slots[0].size.y, self.sequences[0].size.y)
		// return Vec2(width, height) + Vec2(2 * MARGIN) + Vec2(3, 0) * INNER_MARGIN

		let width =
			self.skeleton.sequences[0].get_size(blocs).x + self.skeleton.slots[0].get_size(blocs).x + Self::TEXT_WIDTH;
		let height = self.skeleton.slots[0].get_size(blocs).y.max(self.skeleton.sequences[0].get_size(blocs).y);
		let inner_margin: f64 = todo!("inner margin");
		Vector2::new(width, height) + Vector2::new(2.0, 2.0) * Skeleton::MARGIN + Vector2::new(3.0, 0.0) * inner_margin;
	}

	fn slot_position(&self, slot_id: u16) -> Vector2<f64> {
		// python: `return Vec2(TEXT_While_SIZE.x + INNER_MARGIN, 0) + Vec2(MARGIN)
		let inner_margin = todo!("inner margin");
		// Vector2::new(self.skeleton.size.x + inner_margin, 0.0) + Vector2::new(2.0, 2.0) * Skeleton::MARGIN
	}

	fn sequence_position(&self, sequence_id: u16) -> Vector2<f64> {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}

	fn button_size(&self, button_id: u16) -> Vector2<f64> {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}

	fn button_position(&self, button_id: u16) -> Vector2<f64> {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}

	fn draw_button(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera) {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}

	fn button_function(&mut self, button_id: u16) -> bool {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}
}
