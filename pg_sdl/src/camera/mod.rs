pub struct Camera {
	screen_size: Point, // resolution of the screen
	pub position: Point,
	pub zoom: f32, // scale of the camera
}

impl Camera {
	pub fn new(screen_size: Point) -> Self {
		Camera {
			screen_size,
			position: Point::new(0, 0),
			zoom: 1.0,
		}
	}
}