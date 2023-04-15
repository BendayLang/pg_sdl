#![allow(dead_code, unused_imports, unused_variables)]

use fontdue;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue_sdl2::FontTexture;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};
use std::time::Duration;

mod app;
mod canvas;
mod input;
use app::App;
pub use input::Input;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn draw_text(app: &mut App, font_index: usize, text: &str, x: f32, y: f32) {
    let mut font_texture = FontTexture::new(&app.texture_creator).unwrap();
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        x,
        y,
        ..Default::default()
    });
    layout.append(
        &app.fonts,
        &TextStyle::with_user_data(text, 100.0, font_index, Color::RGB(0xFF, 0xFF, 0)),
    );
    font_texture
        .draw_text(&mut app.canvas, &app.fonts, layout.glyphs())
        .unwrap();
}

fn main() {
    let mut app: App = App::init("benday", 800, 600);

    let mut r = 0;
    let mut text = String::new();

    app.main_loop(&mut |app, _delta| {
        canvas::fill_background(&mut app.canvas, Color::RGB(r, 64, 255 - r));
        canvas::draw_rect(&mut app.canvas, rect!(10, 10, 100, 100), Color::GREEN);

        if let Some(last_char) = app.input.last_char {
            text.push(last_char);
        }
        if app.input.keys_state.backspace == input::KeyState::Pressed {
            text.pop();
        }

        draw_text(app, 1, &text, 100.0, 100.0);

        r = (r + 1) % 255;
    });
}
