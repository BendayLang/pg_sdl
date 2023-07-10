use super::containers::Slot;
use crate::blocs::{Bloc, Skeleton};
use nalgebra::{Point2, Vector2};
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
	// const COLOR: Color = hsv_color(330, 0.3, 1.0);
	const TEXT_SIZE: Vector2<f64> = Vector2::new(30.0, 10.0); // size of "PRINT"

	pub fn new(id: u32, color: Color, position: Point2<f64>) -> Self {
		let slots_positions = Box::new(|slot_id| {
			Vector2::new(Self::TEXT_SIZE.x + Skeleton::INNER_MARGIN, 0.0) + Vector2::new(1.0, 1.0) * Skeleton::MARGIN
		});
		let sequences_positions = Box::new(|_| panic!("no sequences in PrintBloc"));
		let get_size = Box::new(|skeleton: &Skeleton| {
			skeleton.slots[0].get_size()
				+ Vector2::new(Self::TEXT_SIZE.x + Skeleton::INNER_MARGIN, 0.0)
				+ Vector2::new(2.0, 2.0) * Skeleton::MARGIN
		});
		Self {
			skeleton: Skeleton::new(
				id,
				color,
				position,
				vec![Slot::new(color, "value".to_string())],
				slots_positions,
				Vec::new(),
				sequences_positions,
				get_size,
			),
		}
	}
}

impl Bloc for Print {
	fn get_skeleton(&self) -> &Skeleton {
		&self.skeleton
	}
	fn get_skeleton_mut(&mut self) -> &mut Skeleton {
		&mut self.skeleton
	}

	fn button_size(&self, button_id: usize) -> Vector2<f64> {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}

	fn button_position(&self, button_id: usize) -> Vector2<f64> {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}

	fn draw_button(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, camera: &Camera) {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}

	fn button_function(&mut self, button_id: usize) -> bool {
		todo!("With a state pattern, this should not be callable (at compile time)");
	}
}
