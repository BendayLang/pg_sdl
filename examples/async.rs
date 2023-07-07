use color_print::cprintln;
use nalgebra::Vector2;
use pg_sdl::{
	prelude::*,
	widgets::{switch::Switch, text_input::TextInput, Widgets},
};
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time::Duration};

pub struct MyApp {
	camera: Camera,
	background_color: Color,
	sender: mpsc::Sender<String>,
	receiver: mpsc::Receiver<String>,
	mutexed_string: Arc<Mutex<String>>,
}

fn frontend(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>, mutexed_string: Arc<Mutex<String>>) {
	let resolution = Vector2::new(1200, 700);
	let background_color = Colors::SKY_BLUE;
	let camera = Camera::new(resolution, 6, 2.0, 8.0, -5000.0, 5000.0, -5000.0, 5000.0);
	let mut my_app = MyApp { camera, background_color, sender, receiver, mutexed_string };
	let mut pg_sdl: PgSdl = PgSdl::init("Benday", resolution.x, resolution.y, Some(60), true, background_color);
	pg_sdl
		.add_widget("text input", Box::new(TextInput::new(rect!(222, 295, 200, 30), None, None)))
		.add_widget("switch", Box::new(Switch::new(Colors::AMBER, Colors::BLACK, rect!(222, 370, 200, 30), None)))
		.add_widget(
			"button",
			Box::new(Button::new(
				Colors::SKY_BLUE,
				rect!(222, 330, 200, 30),
				None,
				TextStyle::default(),
				"Send".to_string(),
			)),
		);
	pg_sdl.run(&mut my_app);
}

impl App for MyApp {
	fn update(&mut self, _delta: f32, input: &Input, widgets: &mut Widgets) -> bool {
		let mut changed = false;
		changed |= self.camera.update(input);

		match self.receiver.recv_timeout(Duration::from_millis(1)) {
			Ok(received) => {
				cprintln!("<blue>the frontend received: '{}'</blue>", received);
			}
			Err(mpsc::RecvTimeoutError::Disconnected) => panic!("Server disconnected"),
			_ => (),
		}

		let text = widgets.get::<TextInput>("text input").unwrap().content.clone();
		self.mutexed_string.lock().unwrap().clear();
		self.mutexed_string.lock().unwrap().push_str(&text);
		self.mutexed_string.lock().unwrap().push_str(&text);
		if widgets.get_button("button").state.is_pressed() {
			self.sender.send(text).unwrap();
		}

		let switch = widgets.get::<Switch>("switch").unwrap();
		if switch.state.is_pressed() && switch.is_switched() {
			cprintln!("<yellow>mode lag enabled</yellow>");
		} else if switch.state.is_pressed() && !switch.is_switched() {
			cprintln!("<yellow>mode lag disabled</yellow>");
		}
		if switch.is_switched() {
			thread::sleep(Duration::from_millis(80));
		}

		changed
	}

	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		self.camera.draw_grid(canvas, text_drawer, self.background_color, true, false);
	}
}

fn backend(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>, mutexed_string: Arc<Mutex<String>>) {
	loop {
		match receiver.recv_timeout(Duration::from_secs(1)) {
			Ok(received) => {
				cprintln!("<green>received, waiting 1s</green>");
				thread::sleep(Duration::from_secs(1));
				cprintln!("server get: {} (waiting 1s)", received);
				println!("mutexed_string: {}", mutexed_string.lock().unwrap());
				sender.send(format!("{} from backend", received)).unwrap();
			}
			Err(mpsc::RecvTimeoutError::Disconnected) => panic!("Server disconnected"),
			_ => (),
		}
	}
}

fn main() {
	let (txb, rxb) = mpsc::channel();
	let (txf, rxf) = mpsc::channel();
	let mutexed_string = Arc::new(Mutex::new(String::from("value")));
	let mutexed_string_clone = mutexed_string.clone();

	thread::spawn(move || backend(txb, rxf, mutexed_string_clone));
	frontend(txf, rxb, mutexed_string);
}
