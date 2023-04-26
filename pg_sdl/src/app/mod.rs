use std::time::Instant;
use fontdue::layout::{HorizontalAlign, VerticalAlign};

use sdl2::{render::Canvas, video::Window, pixels::Color};
use crate::{Input, MyApp, point, rect};
use crate::canvas::{fill_background};

use crate::text::{Text, TextDrawer};

pub type UpdateFn = fn(&mut MyApp, f32, &Input) -> bool;
pub type DrawFn = fn(&mut MyApp, &mut Canvas<Window>, &mut TextDrawer);

pub struct App {
	pub input: Input,
	pub canvas: Canvas<Window>,
	pub text_drawer: TextDrawer,
	pub background_color: Color,
	fps: Option<f32>,
	draw_fps: bool,
	update: UpdateFn,
	draw: DrawFn,
}


impl App {
	pub fn init(
		window_title: &str,
		window_width: u32,
		window_height: u32,
		fps: Option<f32>,
		draw_fps: bool,
		background_color: Color,
		update: UpdateFn,
		draw: DrawFn
	) -> Self {
		let sdl_context = sdl2::init().unwrap();
		
		let video_subsystem = sdl_context
			.video()
			.expect("SDL video subsystem could not be initialized");
		
		let window = video_subsystem
			.window(window_title, window_width, window_height)
			.position_centered()
			.resizable()
			.build()
			.expect("Window could not be created");
		
		let canvas = window.into_canvas().build().unwrap();
		
		App {
			text_drawer: TextDrawer::new(canvas.texture_creator()),
			input: Input::new(sdl_context),
			canvas,
			background_color,
			fps,
			draw_fps,
			update,
			draw,
		}
	}
	
	pub fn run(&mut self, my_app: &mut MyApp) {
		let mut frame_instant: Instant;
		let mut frame_time: f32 = 0.02;
		
		fill_background(&mut self.canvas, self.background_color);
		(self.draw)(my_app, &mut self.canvas, &mut self.text_drawer);
		
		'running: loop {
			// Time control
			frame_instant = Instant::now();
			
			self.input.get_events();
			if self.input.window_closed { break 'running; }
			
			// Update
			// Draw
			if (self.update)(my_app, frame_time, &self.input) || true {
				fill_background(&mut self.canvas, self.background_color);
				(self.draw)(my_app, &mut self.canvas, &mut self.text_drawer);
			}
			
			// FPS
			if self.draw_fps {
				self.canvas.set_draw_color(Color::WHITE);
				self.canvas.fill_rect(rect!(10.0, 1.0, 120.0, 32.0)).unwrap();
				self.text_drawer.draw(
					&mut self.canvas,
					&Text::new(format!("FPS: {0:.0}", 1.0 / frame_time), 30.0),
					point!(10.0, 1.0),
					None,
					None,
					HorizontalAlign::Left,
					VerticalAlign::Top);
			}
			
			// Render to screen
			self.canvas.present();
			
			// Sleep
			if let Some(fps) = &self.fps {
				let to_sleep = 1.0 / fps - frame_instant.elapsed().as_secs_f32();
				if to_sleep > 0.0 {
					std::thread::sleep(std::time::Duration::from_secs_f32(to_sleep));
				}
			}
			
			frame_time = frame_instant.elapsed().as_secs_f32();
		}
	}
}
