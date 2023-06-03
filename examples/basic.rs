use nalgebra::{Point2, Vector2};
use pg_sdl::prelude::*;
use pg_sdl::widgets::{switch::Switch, text_input::TextInput, Widgets};

// Here we define our app-state struct
pub struct MyApp {
	camera: Camera,
	draw_circle: bool,
	background_color: Color,
}

// To call the run function of PgSdl, we need to implement the App trait for our app-state struct
impl App for MyApp {
	// The update function is called every frame, and is used to update the app-state
	fn update(&mut self, _delta: f32, input: &Input, widgets: &mut Widgets) -> bool {
		let mut changed = false;
		// if not widgets_changed { TODO implement that <-
		changed |= self.camera.update(input);
		// }

		if self.draw_circle {
			changed = true;
			self.draw_circle = false;
		}
		let button = widgets.get_button("button");
		if button.state.is_down() {
			self.draw_circle = true;
			changed = true;
		}
		changed
	}

	// The draw function is called every frame, if update returned true or any widget has changed
	// It is called just after the update function
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		// We can put any custom drawing code here
		if self.draw_circle {
			canvas.set_draw_color(Colors::VIOLET);
			draw_circle(canvas, point!(500, 400), 100, 20);

			canvas.set_draw_color(Colors::LIGHT_RED);
			let width: u32 = 20;
			let rect = rect!(650, 350, 150, 100);
			let rects = (0..width)
				.map(|i| rect!(rect.x as u32 + i, rect.y as u32 + i, rect.width() - 2 * i, rect.height() - 2 * i))
				.collect::<Vec<Rect>>();
			canvas.draw_rects(&rects).unwrap();
		}

		// self.camera.draw_vertical_line(canvas, darker(self.background_color, 0.9), 200.0);
		// self.camera.draw_vertical_line(canvas, darker(self.background_color, 0.7), 220.0);
		self.camera.draw_grid(canvas, text_drawer, self.background_color, true, false);

		let vertices = Vec::from([
			Point2::new(500.0, 200.0),
			Point2::new(600.0, 200.0),
			Point2::new(650.0, 400.0),
			Point2::new(480.0, 380.0),
		]);
		self.camera.fill_polygon(canvas, Colors::LIGHT_BLUE, &vertices);
		self.camera.draw_polygon(canvas, Colors::BLACK, &vertices);

		self.camera.fill_rectangle(canvas, Colors::DARK_CYAN, Point2::new(350.0, 300.0), Vector2::new(80.0, 50.0));
		self.camera.draw_rectangle(canvas, Colors::CYAN, Point2::new(350.0, 300.0), Vector2::new(80.0, 50.0));

		self.camera.fill_ellipse(canvas, Colors::DARK_GREEN, Point2::new(700.0, 300.0), Vector2::new(100.0, 60.0));
		self.camera.draw_ellipse(canvas, Colors::LIGHT_GREEN, Point2::new(700.0, 300.0), Vector2::new(100.0, 60.0));

		self.camera.fill_circle(canvas, Colors::ORANGE, Point2::new(800.0, 300.0), 50.0);
		self.camera.draw_circle(canvas, Colors::DARK_ORANGE, Point2::new(800.0, 300.0), 50.0);

		// All the widgets are drawn automatically by PgSdl above all else
	}
}

fn main() {
	// First we initialize our custom app-state struct
	let resolution = Vector2::new(1200, 700);
	let background_color = Colors::SKY_BLUE;

	let camera = Camera::new(resolution, 6, 2.0, 8.0, -5000.0, 5000.0, -5000.0, 5000.0);
	let mut my_app = MyApp { camera, draw_circle: false, background_color };

	// Then we initialize the PgSdl struct
	let mut pd_sdl: PgSdl = PgSdl::init("Benday", resolution.x, resolution.y, Some(60), true, background_color);

	// We can add widgets to the PgSdl struct (as long as they implement the Widget trait)
	// We will retrieve them later in the update function with the name we gave them
	pd_sdl
		.add_widget(
			"button",
			Box::new(Button::new(
				Colors::ROYAL_BLUE,
				rect!(500, 500, 200, 100),
				Some(9),
				TextStyle::default(),
				"Auto !".to_string(),
			)),
		)
		.add_widget(
			"slider",
			Box::new(Slider::new(
				Colors::ROYAL_BLUE,
				rect!(110, 220, 200, 30),
				Some(9),
				SliderType::Continuous { display: None, default_value: 0.5 },
			)),
		)
		.add_widget(
			"text input",
			Box::new(TextInput::new(
				rect!(222, 295, 200, 30),
				None, // TextInputStyle::default(),
				None, // TextStyle::default(),
			)),
		)
		.add_widget(
			"switch",
			Box::new(Switch::new(Colors::LIGHT_GREEN, Colors::LIGHT_RED, rect!(200, 150, 50, 30), Some(10))),
		)
		.add_widget(
			"switch2",
			Box::new(Switch::new(
				Colors::LIGHT_ORANGE,
				paler(Colors::LIGHT_ORANGE, 0.25),
				rect!(280, 140, 30, 50),
				Some(10),
			)),
		);

	// Finally we run the app, that take a mutable reference to our custom app-state struct
	pd_sdl.run(&mut my_app);
}
