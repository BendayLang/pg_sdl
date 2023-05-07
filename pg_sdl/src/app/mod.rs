use crate::prelude::*;
use crate::widgets::Widgets;
use sdl2::{pixels::Color, render::Canvas, ttf::FontStyle, video::Window};
use std::collections::HashMap;
use std::{fmt::format, time::Instant};

pub trait UserApp {
    fn update(&mut self, delta: f32, input: &Input, widgets: &mut Widgets) -> bool;
    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer);
}

pub struct App {
    pub input: Input,
    pub canvas: Canvas<Window>,
    pub text_drawer: TextDrawer,
    pub background_color: Color,
    widgets: Widgets,
    fps: Option<u32>,
    draw_fps: bool,
}

impl App {
    pub fn init(
        window_title: &str,
        window_width: u32,
        window_height: u32,
        fps: Option<u32>,
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
            .resizable()
            .build()
            .expect("Window could not be created");

        let canvas = window.into_canvas().build().unwrap();

        App {
            text_drawer: TextDrawer::new(canvas.texture_creator()),
            input: Input::new(sdl_context),
            widgets: Widgets::new(),
            canvas,
            background_color,
            fps,
            draw_fps,
        }
    }

    pub fn run<U>(&mut self, user_app: &mut U)
    where
        U: UserApp,
    {
        let mut frame_instant: Instant;
        let mut frame_time: f32 = 0.02;

        self.input.get_events(); // permet au draw de savoir ou placer les widgets la première fois
        fill_background(&mut self.canvas, self.background_color);
        user_app.draw(&mut self.canvas, &mut self.text_drawer);
        self.widgets
            .0
            .iter_mut()
            .for_each(|(_, widget)| widget.draw(&mut self.canvas, &mut self.text_drawer));

        'running: loop {
            // Time control
            frame_instant = Instant::now();

            self.input.get_events();
            if self.input.window_closed {
                break 'running;
            }

            // Update
            // Draw
            let changed = self
                .widgets
                .0
                .iter_mut()
                .any(|(_, widget)| widget.update(&self.input, frame_time));
            if user_app.update(frame_time, &self.input, &mut self.widgets) || changed {
                fill_background(&mut self.canvas, self.background_color);
                user_app.draw(&mut self.canvas, &mut self.text_drawer);
                self.widgets
                    .0
                    .iter_mut()
                    .for_each(|(_, widget)| widget.draw(&mut self.canvas, &mut self.text_drawer));
            }

            // FPS
            if self.draw_fps {
                self.canvas.set_draw_color(Color::WHITE);
                self.canvas
                    .fill_rect(rect!(10.0, 2.0, 120.0, 32.0))
                    .unwrap();
                self.text_drawer.draw(
                    &mut self.canvas,
                    point!(65.0, 17.0),
                    &Text::new(format!("FPS: {0:.0}", 1.0 / frame_time), 24, None),
                );
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

            frame_time = frame_instant.elapsed().as_secs_f32();
        }
    }

    pub fn add_widget(&mut self, name: &str, widget: Box<dyn Widget>) {
        self.widgets.add(name, widget);
    }

    pub fn add_widgets(&mut self, widgets: HashMap<&str, Box<dyn Widget>>) {
        for (name, widget) in widgets {
            self.widgets.add(name, widget);
        }
    }
}
