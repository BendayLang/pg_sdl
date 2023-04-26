use sdl2::rect::Point;
use crate::{Input, point};

pub struct Camera {
	resolution: Point,
	pub position: Point,
	pub scale: f32,
}

impl Camera {
	pub fn new(resolution: Point) -> Self {
		Camera {
			resolution,
			position: Point::new(0, 0),
			scale: 1.0,
		}
	}
	
	pub fn update(&mut self, input: &Input) -> bool {
		let changed = false;
		
		changed
	}
	
	fn resize(&mut self, new_size: Point) {
		// self.move((self.resolution - new_size) / self.scale / 2);
		self.resolution = new_size;
	}
	
	fn screen2world(self, point: Point) -> Point {
		// Renvoie la position d'un point à l'écran en position dans le monde.
		point!(
			point.x as f32 / self.scale + self.position.x as f32,
			point.y as f32 / self.scale + self.position.y as f32)
	}
	
	fn world2screen(self, point: Point) -> Point {
		// Renvoie la position d'un point dans le monde en position à l'écran.
		point!(
			(point.x - self.position.x) as f32 * self.scale,
			(point.y - self.position.y) as f32 * self.scale)
	}
}
