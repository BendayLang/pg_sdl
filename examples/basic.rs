use pg_sdl::prelude::*;
use pg_sdl::widgets::text_input::TextInput;
use pg_sdl::widgets::Widgets;

pub struct MyApp {
    pub draw_circle: bool,
}

impl App for MyApp {
    fn update(&mut self, _delta: f32, _input: &Input, widgets: &mut Widgets) -> bool {
        let mut changed = false;
        if self.draw_circle {
            changed = true;
            self.draw_circle = false;
        }
        if widgets.get_button("button").state.is_down() {
            self.draw_circle = true;
            changed = true;
        }
        changed
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, _text_drawer: &mut TextDrawer) {
        if self.draw_circle {
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
        }
    }
}

fn main() {
    let mut my_app = MyApp { draw_circle: false };
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
