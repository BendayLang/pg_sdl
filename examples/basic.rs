#![allow(dead_code, unused_imports, unused_variables)]

use pg_sdl::prelude::*;
use pg_sdl::widgets::text_input::TextInput;
use pg_sdl::widgets::Widgets;
use pg_sdl::{get_button, get_button_mut, get_widget};
use std::collections::HashMap;

pub struct MyApp;

impl App for MyApp {
    fn update(&mut self, delta: f32, input: &Input, widgets: &mut Widgets) -> bool {
        if get_button!(widgets, "button").state.is_pressed() {
            println!("Button pressed !");
        }
        false
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        canvas.set_draw_color(Colors::VIOLET);
        draw_circle(canvas, point!(500, 400), 100, 20);

        canvas.set_draw_color(Colors::LIGHT_RED);
        let width: u32 = 20;
        let rect = rect!(650, 350, 150, 100);
        let rects = (0..width)
            .map(|i| {
                rect!(
                    rect.x as u32 + i,
                    rect.y as u32 + i,
                    rect.width() - 2 * i,
                    rect.height() - 2 * i
                )
            })
            .collect::<Vec<Rect>>();
        canvas.draw_rects(&rects).unwrap();

        canvas.set_draw_color(Colors::BLACK);
        let center = point!(500, 400);
    }
}

fn main() {
    let mut my_app = MyApp;

    let mut pd_sdl: PgSdl = PgSdl::init("Benday", 1200, 720, Some(60), true, Colors::SKY_BLUE);

    pd_sdl
        .add_widget(
            "button",
            Box::new(Button::new(
                Colors::ROYAL_BLUE,
                rect!(500, 500, 200, 100),
                Some(9),
                Some(Text::new("Auto !".to_string(), 16, None)),
            )),
        )
        .add_widget(
            "slider",
            Box::new(Slider::new(
                Colors::ROYAL_BLUE,
                rect!(110, 220, 200, 100),
                Some(9),
                SliderType::Continuous {
                    display: None,
                    default_value: 0.5,
                },
            )),
        )
        .add_widget(
            "text input",
            Box::new(TextInput::new(
                Colors::WHITE,
                rect!(222, 295, 200, 100),
                Some(9),
                Some(Text::new("Auto !".to_string(), 16, None)),
            )),
        );

    pd_sdl.run(&mut my_app);
}
