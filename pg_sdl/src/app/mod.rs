use std::time::Instant;

use sdl2::{render::Canvas, video::Window, pixels::Color};
use crate::point;

use crate::text::TextDrawer;

pub struct App {
    pub input: crate::input::Input,
    pub canvas: Canvas<Window>,
    pub text_drawer: TextDrawer,
    pub background_color: Color,
    fps: f32,
    draw_fps: bool,
}

impl App {
    pub fn init(
        window_title: &str,
        window_width: u32,
        window_height: u32,
        fps: u32,
        draw_fps: bool,
        background_color: Color,
    ) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context
            .video()
            .expect("SDL video subsystem could not be initialized");
        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()
            .expect("Window could not be created");
        let canvas = window.into_canvas().build().unwrap();
        App {
            text_drawer: TextDrawer::new(canvas.texture_creator()),
            input: crate::input::Input::new(sdl_context),
            canvas,
            background_color: background_color,
            fps: fps as f32,
            draw_fps,
        }
    }

    pub fn main_loop<G>(&mut self, update: &mut G)
        where
            G: FnMut(&mut App, f32) -> (),
    {
        let mut last_update_time: Instant;
        let mut frame_count = 0;
        let mut frame_time = std::time::Instant::now();
        let mut frame_rate = 0;
        let mut update_start;
        last_update_time = std::time::Instant::now();

        'running: loop {
            {
                // Input
                self.input.get_events();
                if self.input.window_closed {
                    break 'running;
                }
            }

            // Background color
            crate::canvas::fill_background(&mut self.canvas, self.background_color);

            {
                // Update
                update_start = std::time::Instant::now();
                update(self, last_update_time.elapsed().as_secs_f32());
                last_update_time = std::time::Instant::now();
            }

            // FPS
            if self.draw_fps {
                frame_count += 1;
                if frame_time.elapsed().as_secs_f32() > 1.0 / 3.0 {
                    frame_rate = frame_count * 3;
                    frame_count = 0;
                    frame_time = std::time::Instant::now();
                }
                self.text_drawer.draw(
                    &mut self.canvas,
                    1,
                    &format!("FPS: {}", frame_rate),
                    point!(10.0, 1.0),
                    30.0,
                    Color::BLACK
                );
            }

            {
                // Render and sleep
                self.canvas.present();
                let to_sleep = 1.0 / self.fps - update_start.elapsed().as_secs_f32();
                if to_sleep > 0.0 {
                    std::thread::sleep(std::time::Duration::from_secs_f32(to_sleep));
                }
            }
        }
    }
}
