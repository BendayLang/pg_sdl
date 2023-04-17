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

fn main() {
    let mut app: App = App::init("benday", 800, 600, 60, true);

    app.main_loop(&mut |app, _delta| {
        app.background_color = Color::BLACK;
        app.text_drawer
            .draw_text(&mut app.canvas, 1, "salut !", 130.0, 130.0, 20.0);
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
    });
}
