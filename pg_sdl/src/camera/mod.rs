use crate::{input::Input, point};
use nalgebra::{Point2, Scale2, Similarity2, Translation2, Vector2};
use sdl2::rect::Point;

pub struct Camera {
	resolution: Vector2<u32>,
	pub similarity: Similarity2<f64>,
}

impl Camera {
	const SCALING_FACTOR: f64 = 1.1892071150027210667174999705605; // f64::powf(2.0, 1.0 / 4.0);

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

		let scroll = input.mouse.wheel;
		if scroll != 0 {
			let scaling = Self::SCALING_FACTOR.powf(scroll as f64);
			let center = Point2::from(Vector2::new(input.mouse.position.x as f64, input.mouse.position.y as f64));

			let translation = Translation2::from((1.0 / scaling - 1.0) * center.coords);
			self.similarity.append_translation_mut(&translation);
			self.similarity.append_scaling_mut(scaling);
			changed = true;
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
