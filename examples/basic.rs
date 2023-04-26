#![allow(dead_code, unused_imports, unused_variables)]

extern crate pg_sdl;
use pg_sdl::prelude::*;
use std::collections::HashMap;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    font: &mut sdl2::ttf::Font,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font_style: Option<&sdl2::ttf::FontStyle>,
) {
    if let Some(font_style) = font_style {
        font.set_style(*font_style);
    }

    // render a surface, and convert it to a texture bound to the canvas
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

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 0;
    let target = get_centered_rect(
        width,
        height,
        SCREEN_WIDTH - padding,
        SCREEN_HEIGHT - padding,
    );

    canvas.copy(&texture, None, Some(target)).unwrap();
}

pub struct MyApp {
    buttons: Vec<Button>,
    sliders: Vec<Slider<i32>>,
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
        // Tout ce qui pourrais être fait dans le constructeur (1 seule fois)
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let texture_creator = canvas.texture_creator();
        let mut font = ttf_context
            .load_font(Path::new("/usr/share/fonts/TTF/Vera.ttf"), 28)
            .unwrap();
        // La fn draw_text
        draw_text(canvas, &mut font, &texture_creator, None);

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

        // let text = self.text.clone();
        // text_drawer.draw(
        //     canvas,
        //     &Text {
        //         text,
        //         color: Colors::BLUE,
        //         ..Default::default()
        //     },
        //     point!(130.0, 250.0),
        //     None,
        //     None,
        //     HorizontalAlign::Left,
        //     VerticalAlign::Top,
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
                Some(Text::new("Hello World !".to_string(), 16.0)),
            ),
            Button::new(Colors::GREY, rect!(550, 20, 80, 50), Some(7), None),
        ],
        sliders: vec![],
        // sliders: vec![
        //     Slider::new(
        //         Colors::GREEN,
        //         rect!(500, 150, 180, 18),
        //         Some(4),
        //         SliderType::Discret {
        //             snap: 10,
        //             default_value: 5,
        //         },
        //         Box::new(|value| (value * 10.0) as i32),
        //         Some(Box::new(|value| format!("{}", (value - 5) * 2))),
        //     ),
        //     Slider::new(
        //         Colors::ORANGE,
        //         rect!(700, 80, 30, 150),
        //         Some(14),
        //         SliderType::Continuous { default_value: 0.2 },
        //         Box::new(|value: f32| (value * 100.0) as i32),
        //         Some(Box::new(|value| format!("{}%", value))),
        //     ),
        // ],
        blocs: HashMap::from([(0, Bloc::new(Colors::MAGENTA, rect!(120, 230, 110, 80)))]),
        text: String::new(),
    };

    let mut app: App = App::init("Benday", 1200, 720, Some(60.0), true, Colors::SKY_BLUE);

    app.run(&mut my_app);
}
