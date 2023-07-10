use crate::prelude::*;
use crate::widgets::Widgets;
use ndarray::AssignElem;
use sdl2::mouse::{Cursor, MouseUtil, SystemCursor};
use sdl2::ttf::FontStyle;
use sdl2::{pixels::Color, render::Canvas, video::Window};
use std::collections::HashMap;
use std::time::Instant;

pub trait App {
	fn update(&mut self, delta: f64, input: &Input, widgets: &mut Widgets) -> bool;
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer);
}

pub struct PgSdl {
	mouse: MouseUtil,
	input: Input,
	canvas: Canvas<Window>,
	text_drawer: TextDrawer,
	background_color: Color,
	widgets: Widgets,
	fps: Option<u32>,
	draw_fps: bool,
}

impl PgSdl {
	pub fn init(
		window_title: &str, window_width: u32, window_height: u32, fps: Option<u32>, draw_fps: bool,
		background_color: Color,
	) -> Self {
		let sdl_context = sdl2::init().expect("SDL could not be initialized");

		let video_subsystem = sdl_context.video().expect("SDL video subsystem could not be initialized");

		video_subsystem.text_input().start();

		let window = video_subsystem
			.window(window_title, window_width, window_height)
			.position_centered()
			.resizable()
			.build()
			.expect("Window could not be created");

		let canvas = window.into_canvas().build().expect("Canvas could not be created");

		PgSdl {
			mouse: sdl_context.mouse(),
			text_drawer: TextDrawer::new(canvas.texture_creator()),
			input: Input::new(sdl_context, video_subsystem.clipboard()),
			widgets: Widgets::new(),
			canvas,
			background_color,
			fps,
			draw_fps,
		}
	}

	fn draw_fps(&mut self, delta: f64) {
		self.canvas.set_draw_color(Color::WHITE);
		self.canvas.fill_rect(rect!(10.0, 2.0, 120.0, 32.0)).unwrap();
		self.text_drawer.draw(
			&mut self.canvas,
			point!(65.0, 17.0),
			&TextStyle::new(24, None, Color::BLACK, FontStyle::NORMAL),
			&format!("FPS: {0:.0}", 1.0 / delta),
			Align::Center,
		);
	}

	fn draw<U>(&mut self, user_app: &U)
	where
		U: App,
	{
		self.canvas.set_draw_color(self.background_color);
		self.canvas.clear();
		user_app.draw(&mut self.canvas, &mut self.text_drawer);
		self.widgets.draw(&mut self.canvas, &self.text_drawer);
	}

	fn update<U>(&mut self, user_app: &mut U, delta: f64) -> bool
	where
		U: App,
	{
		let mut changed = self.widgets.update(&self.input, delta, &mut self.text_drawer);
		changed |= user_app.update(delta, &self.input, &mut self.widgets);
		changed
	}

	pub fn run<U>(&mut self, user_app: &mut U)
	where
		U: App,
	{
		let mut frame_instant: Instant;
		let mut frame_time: f64 = 0.02;

		self.input.get_events(); // permet au draw de savoir ou placer les widgets la première fois
		self.draw(user_app);

		'running: loop {
			// Time control
			frame_instant = Instant::now();

			self.input.get_events();
			if self.input.window_closed {
				break 'running;
			}

			// Update
			// Draw
			if self.update(user_app, frame_time) {
				self.draw(user_app);
			}

			// FPS
			if self.draw_fps {
				self.draw_fps(frame_time);
			}

			// Render to screen
			self.canvas.present();

			// Sleep
			if let Some(fps) = &self.fps {
				let to_sleep = 1.0 / *fps as f32 - frame_instant.elapsed().as_secs_f32();
				if to_sleep > 0.0 {
					std::thread::sleep(std::time::Duration::from_secs_f32(to_sleep));
				}
			}

			frame_time = frame_instant.elapsed().as_secs_f64();
		}
	}

	pub fn add_widget(&mut self, name: &str, widget: Box<dyn Widget>) -> &mut Self {
		self.widgets.add(name, widget);
		self
	}

	pub fn add_widgets(&mut self, widgets: HashMap<&str, Box<dyn Widget>>) {
		for (name, widget) in widgets {
			self.widgets.add(name, widget);
		}
	}

	pub fn change_mouse_cursor(&mut self) {
		let cursor = Cursor::from_system(SystemCursor::WaitArrow).expect("mouse cursor loading error");
		cursor.set();
		// self.mouse
	}
}
