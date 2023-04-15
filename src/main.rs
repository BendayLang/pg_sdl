#![allow(dead_code, unused_imports, unused_variables)]

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    surface::SurfaceRef,
    ttf::Font,
    video::{Window, WindowContext},
};
use std::path::Path;
use std::time::Duration;

mod canvas;
mod input;
use canvas::WindowCanvas;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn draw_text(
    texture_creator: &TextureCreator<WindowContext>,
    font: &mut Font,
    font_style: sdl2::ttf::FontStyle,
    canvas: &mut WindowCanvas,
) {
    font.set_style(font_style);
    let surface = font
        .render("Hello Rust!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())
        .unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap();

    let TextureQuery { width, height, .. } = texture.query();
    canvas
        .0
        .copy(&texture, None, Some(rect!(0, 0, width, height)))
        .unwrap();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let video_subsystem = sdl_context
        .video()
        .expect("SDL video subsystem could not be initialized");
    let window = video_subsystem
        .window("Oxy", 800, 600)
        .position_centered()
        .build()
        .expect("Window could not be created");
    let mut canvas = WindowCanvas(window.into_canvas().build().unwrap());

    let mut input = input::Input::new(sdl_context);

    let texture_creator = canvas.0.texture_creator();
    let mut font = ttf_context
        .load_font(Path::new(r#"/usr/share/fonts/TTF/VeraBd.ttf"#), 120)
        .unwrap();

    let mut i = 0;
    'running: loop {
        input.get_events();
        if input.should_quit || input.keys_state.esc == input::KeyState::Pressed {
            break 'running;
        }

        i = (i + 1) % 255;
        canvas.fill_background(Color::RGB(i, 64, 255 - i));
        canvas.draw_rect(rect!(10, 10, 100, 100), Color::GREEN);
        draw_text(
            &texture_creator,
            &mut font,
            sdl2::ttf::FontStyle::BOLD,
            &mut canvas,
        );
        canvas.0.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
