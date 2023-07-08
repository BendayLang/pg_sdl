use itertools::Itertools;
use nalgebra::{Point2, Vector2};
use pg_sdl::prelude::*;
use pg_sdl::widgets::{switch::Switch, text_input::TextInput, Widgets};
use rand;

#[derive(Copy, Clone)]
struct Ant {
	pub x: f64,
	pub direction_right: bool,
	pub color: Color,
}

// Here we define our app-state struct
pub struct MyApp {
	camera: Camera,
	background_color: Color,
	ants: Vec<Ant>,
}

// To call the run function of PgSdl, we need to implement the App trait for our app-state struct
impl App for MyApp {
	// The update function is called every frame, and is used to update the app-state
	fn update(&mut self, delta: f64, input: &Input, widgets: &mut Widgets) -> bool {
		let gspeed = widgets.get_slider("slider").get_value() as f64 * delta;

		let mut change = Vec::new();
		self.ants.iter().enumerate().combinations(2).for_each(|tants| {
			let (i1, ant1) = tants.get(0).unwrap();
			let (i2, ant2) = tants.get(1).unwrap();
			let ant1_v = gspeed * if ant1.direction_right { 1.0 } else { -1.0 };
			let ant2_v = gspeed * if ant2.direction_right { 1.0 } else { -1.0 };

			if (ant1.direction_right != ant2.direction_right) {
				if ant1.direction_right && (ant1.x < ant2.x) && (ant1.x + ant1_v > ant2.x + ant2_v) {
					change.push(*i1);
					change.push(*i2);
				} else if !ant1.direction_right && (ant1.x > ant2.x) && (ant1.x + ant1_v < ant2.x + ant2_v) {
					change.push(*i1);
					change.push(*i2);
				}
			}
		});

		self.ants.iter_mut().enumerate().for_each(|(i, ant)| {
			if (change.contains(&i)) {
				ant.direction_right = !ant.direction_right;
			}
		});

		for ant in &mut self.ants {
			let speed = gspeed * if ant.direction_right { 1.0 } else { -1.0 };
			ant.x += speed;
		}
		true
	}

	// The draw function is called every frame, if update returned true or any widget has changed
	// It is called just after the update function
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		// We can put any custom drawing code here

		let origin = Point2::new(400.0, 300.0);
		let d = 600.0;
		DrawRenderer::thick_line(
			canvas,
			origin.x as i16,
			origin.y as i16,
			(origin.x + d) as i16,
			origin.y as i16,
			8,
			Colors::GREY,
		)
		.unwrap();

		for ant in &self.ants {
			let position = origin + Vector2::new(ant.x * d, 0.0);
			DrawRenderer::filled_circle(canvas, position.x as i16, position.y as i16, 5, ant.color);
		}
	}
}

fn main() {
	// First we initialize our custom app-state struct
	let resolution = Vector2::new(1200, 700);
	let background_color = Colors::SKY_BLUE;

	let camera = Camera::new(resolution, 6, 2.0, 8.0, -5000.0, 5000.0, -5000.0, 5000.0);
	let ants = (0..50)
		.map(|i| {
			let color = hsv_color((rand::random::<f64>() * 360.0) as u16, 1.0, 1.0);
			let dir: bool = rand::random();
			Ant { x: rand::random(), direction_right: dir, color }
		})
		.collect();
	let mut my_app = MyApp { camera, background_color, ants };

	// Then we initialize the PgSdl struct
	let mut pd_sdl: PgSdl = PgSdl::init("Benday", resolution.x, resolution.y, Some(60), true, background_color);

	// We can add widgets to the PgSdl struct (as long as they implement the Widget trait)
	// We will retrieve them later in the update function with the name we gave them
	pd_sdl.add_widget(
		"slider",
		Box::new(Slider::new(
			Colors::ROYAL_BLUE,
			rect!(110, 220, 200, 30),
			Some(9),
			SliderType::Continuous { display: None, default_value: 0.0 },
		)),
	);

	// Finally we run the app, that take a mutable reference to our custom app-state struct
	pd_sdl.run(&mut my_app);
}
