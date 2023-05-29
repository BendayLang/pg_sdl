use pg_sdl::prelude::*;
use pg_sdl::widgets::text_input::TextInput;
use pg_sdl::widgets::Widgets;

// Here we define our app-state struct
pub struct MyApp {
	pub draw_circle: bool,
}

// To call the run function of PgSdl, we need to implement the App trait for our app-state struct
impl App for MyApp {
	// The update function is called every frame, and is used to update the app-state
	fn update(&mut self, _delta: f32, _input: &Input, widgets: &mut Widgets) -> bool {
		let mut changed = false;
		if self.draw_circle {
			changed = true;
			self.draw_circle = false;
		}
		let button = widgets.get_button("button");
		if button.state.is_down() {
			self.draw_circle = true;
			changed = true;
		}
		let slider = widgets.get_slider("slider");
		if slider.state.is_down() {
			println!("Slider value: {}", slider.get_value());
		}
		changed
	}

	// The draw function is called every frame, if update returned true or any widget has changed
	// It is called just after the update function
	fn draw(&mut self, canvas: &mut Canvas<Window>, _text_drawer: &mut TextDrawer) {
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
		// All the widgets are drawn automatically by PgSdl
	}
}

fn main() {
	// First we initialize our custom app-state struct
	let mut my_app = MyApp { draw_circle: false };

	// Then we initialize the PgSdl struct
	let mut pd_sdl: PgSdl = PgSdl::init("Benday", 1200, 720, Some(60), true, Colors::SKY_BLUE);

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
				rect!(110, 220, 200, 100),
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
		);

	// Finally we run the app, that take a mutable reference to our custom app-state struct
	pd_sdl.run(&mut my_app);
}
