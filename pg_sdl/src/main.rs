#![allow(dead_code, unused_imports, unused_variables)]

use sdl2::pixels::Color;

mod app;
mod canvas;
mod draw_circle;
mod input;
mod text;
mod utils;
use app::App;
pub use input::Input;

struct E {
    x: i32,
    y: i32,
}

fn main() {
    let mut app: App = App::init("benday", 800, 600, 120, true);

    let mut r = 0;
    let mut text = String::new();
    let mut radius = 0.0;

    app.main_loop(&mut |app, _delta| {
        app.set_background_color(Color::RGB(r, 64, 255 - r));

        if radius < 1.0 {
            radius += 0.1 * _delta;
        }
        canvas::fill_rect(
            &mut app.canvas,
            rect!(10, 10, 500, 500),
            Color::GREEN,
            Some(radius),
        );

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
            .draw_text(&mut app.canvas, 1, &text, 130.0, 130.0, 20.0);

        draw_circle::fill_circle(
            &mut app.canvas,
            point!(app.input.mouse.position.x, app.input.mouse.position.y),
            40,
            if app.input.mouse.left_button == input::KeyState::Down {
                Color::BLUE
            } else if app.input.mouse.right_button == input::KeyState::Down {
                Color::YELLOW
            } else if app.input.mouse.middle_button == input::KeyState::Down {
                Color::GREEN
            } else {
                Color::WHITE
            },
        );

        let to_add = (_delta * 20.) as u8;
        r = (r + to_add) % 255;
    });
}
