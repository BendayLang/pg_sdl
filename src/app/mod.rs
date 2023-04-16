use std::time::Instant;

use sdl2::{render::Canvas, video::Window};

use crate::draw_text;

pub struct App {
    pub new_background_color: Option<sdl2::pixels::Color>,
    pub input: crate::input::Input,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub fonts: Vec<fontdue::Font>,
    fps: f32,
    draw_fps: bool,
}

fn fonts_init() -> Vec<fontdue::Font> {
    let font = include_bytes!("/usr/share/fonts/TTF/VeraBd.ttf") as &[u8];
    let vera_bd = fontdue::Font::from_bytes(font, Default::default()).unwrap();
    let font = include_bytes!("/usr/share/fonts/TTF/VeraIt.ttf") as &[u8];
    let vera_it = fontdue::Font::from_bytes(font, Default::default()).unwrap();
    vec![vera_it, vera_bd]
}

impl App {
    pub fn init(
        window_title: &str,
        window_width: u32,
        window_height: u32,
        fps: u32,
        draw_fps: bool,
    ) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
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
            texture_creator: canvas.texture_creator(),
            input: crate::input::Input::new(sdl_context),
            video_subsystem,
            canvas,
            ttf_context,
            fonts: fonts_init(),
            new_background_color: None,
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
            if let Some(color) = self.new_background_color {
                crate::canvas::fill_background(&mut self.canvas, color);
                self.new_background_color = None;
            }

            {
                // Update
                update_start = std::time::Instant::now();
                update(self, last_update_time.elapsed().as_secs_f32());
                last_update_time = std::time::Instant::now();
            }

            {
                // FPS
                frame_count += 1;
                if frame_time.elapsed().as_secs_f32() > 1.0 / 3.0 {
                    frame_rate = frame_count * 3;
                    frame_count = 0;
                    frame_time = std::time::Instant::now();
                }
                draw_text::draw_text(self, 1, &format!("FPS: {}", frame_rate), 10.0, 1.0, 30.0);
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
