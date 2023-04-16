#![allow(dead_code, unused_imports, unused_variables)]

use sdl2::pixels::Color;

mod app;
mod canvas;
mod draw_circle;
mod input;
mod text;
mod utils;
mod widgets;

use app::App;
pub use input::Input;
use crate::widgets::Button;
use crate::text::Text;

fn main() {
    let mut app: App = App::init("benday", 800, 600, 60, true, Color::RGB(135, 206, 235));

    let mut button = Button::new(
        Color::RED,
        rect!(100, 100, 200, 100),
        Some(Text{
            text: "Bob".to_string(),
                ..Default::default()
        })
        );
    let mut button2 = Button{
        color: Color::GREEN,
        ..Default::default()
    };

    let mut r = 0;
    let mut text = String::new();

    app.main_loop(&mut |app, delta| {

        button.update(&app.input, delta);
        button.draw(&mut app.canvas, &mut app.text_drawer);

        canvas::fill_rect(
            &mut app.canvas,
            rect!(500, 500, 200, 350),
            Color::RED,
            Some(0.3),
        );

        if let Some(last_char) = app.input.last_char {
            text.push(last_char);
        }
        if app.input.keys_state.backspace == input::KeyState::Pressed {
            text.pop();
        }

        app.text_drawer
            .draw(&mut app.canvas, 1, &text, point!(130.0, 130.0), 20.0, Color::BLUE);

        let to_add = (delta * 20.) as u8;
        r = (r + to_add) % 255;
    });
}
