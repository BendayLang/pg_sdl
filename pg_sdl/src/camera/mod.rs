use crate::{input::Input, point};
use nalgebra::{Point2, Similarity2, Translation2, Vector2};
use sdl2::rect::Point;

pub struct Camera {
	resolution: Vector2<u32>,
	pub similarity: Similarity2<f64>,
}

impl Camera {
	pub fn new(resolution: Vector2<u32>) -> Self {
		Camera { resolution, similarity: Similarity2::identity() }
	}

	pub fn update(&mut self, input: &Input) -> bool {
		let mut changed = false;

		if input.mouse.left_button.is_down() {
			let delta = Vector2::new(input.mouse.delta.x as f64, input.mouse.delta.y as f64);
			if delta != Vector2::zeros() {
				let translation = Translation2::new(delta.x, delta.y);
				self.similarity.append_translation_mut(&translation);
				changed = true;
			}
		}

		changed
	}

	fn resize(&mut self, new_resolution: Vector2<u32>) {
		// self.move((self.resolution - new_size) / self.scale / 2);
		self.resolution = new_resolution;
	}

	/// Renvoie la position d'un point à l'écran en position dans le monde.
	pub fn screen2world(self, point: Point2<f64>) -> Point2<f64> {
		self.similarity * point
	}

	/// Renvoie la position d'un point dans le monde en position à l'écran.
	pub fn world2screen(self, point: Point2<f64>) -> Point2<f64> {
		self.similarity.inverse() * point
	}
}
