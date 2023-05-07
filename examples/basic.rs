#![allow(dead_code, unused_imports, unused_variables)]

use pg_sdl::prelude::*;
use pg_sdl::widgets::Widgets;
use std::collections::HashMap;

pub struct MyApp;

impl UserApp for MyApp {
    fn update(&mut self, delta: f32, input: &Input, widgets: &mut Widgets) -> bool {
        let button = widgets.get_mut::<Button>("button").unwrap();
        if button.state.is_pressed() {
            println!("Button pressed !");
        }
        false
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        canvas.set_draw_color(Colors::VIOLET);
        draw_circle(canvas, point!(500, 400), 100, 20);

        canvas.set_draw_color(Colors::RED_ORANGE);
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

    let mut app: App = App::init("Benday", 1200, 720, Some(60), true, Colors::SKY_BLUE);

    app.add_widget(
        "button",
        Box::new(Button::new(
            Colors::ROYAL_BLUE,
            rect!(500, 500, 200, 100),
            Some(9),
            Some(Text::new("Auto !".to_string(), 16, None)),
        )),
    );

    app.add_widgets(HashMap::from([
        (
            "button2",
            Box::new(Button::new(
                Colors::ROYAL_BLUE,
                rect!(0, 0, 200, 100),
                Some(9),
                Some(Text::new("Auto !".to_string(), 16, None)),
            )) as Box<dyn Widget>,
        ),
        (
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
        ),
    ]));

    app.run(&mut my_app);
}
