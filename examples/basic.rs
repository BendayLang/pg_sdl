#![allow(dead_code, unused_imports, unused_variables)]

extern crate pg_sdl;

use pg_sdl::prelude::*;
use std::collections::HashMap;

use pg_sdl::canvas::draw_rect;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::ttf::FontStyle;



pub struct MyApp {
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
    blocs: HashMap<u32, Bloc>,
    text: String,
}

impl MyApp {
    fn widgets(&mut self) -> Vec<&mut dyn Widget> {
        self.buttons
            .iter_mut()
            .map(|button| button as &mut dyn Widget)
            .chain(
                self.sliders
                    .iter_mut()
                    .map(|slider| slider as &mut dyn Widget),
            )
            .collect()
    }
}

impl UserApp for MyApp {
    fn update(&mut self, delta: f32, input: &Input) -> bool {
        let mut changed = self
            .widgets()
            .iter_mut()
            .any(|widget| widget.update(&input, delta));

        if self.buttons[0].state.is_pressed() {
            // println!("{}", self.sliders[0].get_value());
        }

        if let Some(last_char) = input.last_char {
            self.text.push(last_char);
            changed = true;
        };
        if input.keys_state.backspace.is_pressed() {
            if let Some(_) = self.text.pop() {
                changed = true;
            }
        };

        changed
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        let widgets = self
            .buttons
            .iter_mut()
            .map(|button| button as &mut dyn Widget)
            .chain(
                self.sliders
                    .iter_mut()
                    .map(|slider| slider as &mut dyn Widget),
            )
            .collect::<Vec<&mut dyn Widget>>();

        for widget in widgets {
            widget.draw(canvas, text_drawer);
        }
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

        for (_id, bloc) in &self.blocs {
            bloc.draw(canvas, text_drawer);
        }

        canvas.set_draw_color(Colors::BLACK);
        let center = point!(500, 400);

        // draw_text(
        //     canvas,
        //     center,
        //     "bob le bricoleur",
        //     "DejaVuSans",
        //     40,
        //     sdl2::ttf::FontStyle::NORMAL,
        //     Colors::BLACK,
        // );
    }
}

fn main() {
    let mut my_app = MyApp {
        buttons: vec![
            Button::new(
                Colors::ROYAL_BLUE,
                rect!(100, 100, 200, 100),
                Some(9),
                Some(Text::new("Hello World !".to_string(), 16, None)),
            ),
            Button::new(Colors::GREY, rect!(550, 20, 80, 50), Some(7), None),
        ],
        sliders: vec![],
        blocs: HashMap::from([(0, Bloc::new(Colors::MAGENTA, rect!(120, 230, 110, 80)))]),
        text: String::new(),
    };

    let mut app: App = App::init("Benday", 1200, 720, Some(60.0), true, Colors::SKY_BLUE);

    app.run(&mut my_app);
}
