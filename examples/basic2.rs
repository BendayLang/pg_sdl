#![allow(dead_code, unused_variables)]

extern crate pg_sdl;
use std::collections::HashMap;
use pg_sdl::prelude::*;

enum AppState {
    Idle,
    Selected { id: u32 },
}

pub struct MyApp {
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
    id_counter: u32,
    bloc_state: AppState,
    blocs: HashMap<u32, Bloc>,
    text: String,
}

impl UserApp for MyApp {
    fn update(&mut self, delta: f32, input: &Input) -> bool {
        let mut changed = false;

        let widgets: Vec<&mut dyn Widget> = self
            .buttons
            .iter_mut()
            .map(|button| button as &mut dyn Widget)
            .chain(
                self.sliders
                    .iter_mut()
                    .map(|slider| slider as &mut dyn Widget),
            )
            .collect();
        for widget in widgets {
            changed |= widget.update(&input, delta);
        }

        if self.buttons[2].state.is_pressed() {
            self.sliders[0].reset_value();
        }

        match self.bloc_state {
            AppState::Idle => {
                // Add a bloc
                if self.buttons[0].state.is_pressed() {
                    let id = self.id_counter;
                    self.id_counter += 1;
                    self.blocs.insert(
                        id,
                        Bloc::new(
                            hsv_color((id * 30) as u16, 1.0, 1.0),
                            rect!(10 * id + 120, 10 * id + 230, 110, 80),
                        ),
                    );
                }
                // Select a bloc
                if input.mouse.left_button.is_pressed() {
                    let mouse_pos = input.mouse.position;
                    for (id, bloc) in &mut self.blocs {
                        if bloc.collide(mouse_pos) {
                            self.bloc_state = AppState::Selected { id: *id };
                            changed = true;
                        }
                    }
                }
            }
            AppState::Selected { id: moving_bloc_id } => {
                // Move a bloc
                self.blocs
                    .get_mut(&moving_bloc_id)
                    .unwrap()
                    .move_by(input.mouse.delta);
                changed |= input.mouse.delta != point!(0, 0);

                if input.mouse.left_button.is_released() {
                    let moving_bloc = self.blocs.get(&moving_bloc_id).unwrap();
                    // let mut id_bloc: Option<(&u32, &mut Bloc)> = self
                    //     .blocs
                    //     .iter_mut()
                    //     .find(|(_id, bloc)| moving_bloc.collide_bloc(bloc));

                    // // id_bloc.unwrap().1.set_child(moving_bloc_id);
                    // if let Some((id, parent_bloc)) = id_bloc {
                    //     // let parent_bloc = self.blocs.get_mut(&id).unwrap();
                    //     parent_bloc.set_child(moving_bloc_id);
                    // }

                    changed = true;
                    self.bloc_state = AppState::Idle;
                }
            }
        }

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

        let text = self.text.clone();
        // text_drawer.draw(
        //     canvas,
        //     &Text::new(text, 20.0),
        //     point!(130.0, 250.0),
        //     None,
        //     None,
        //     HorizontalAlign::Left,
        //     VerticalAlign::Top,
        // );
    }
}

fn main() {
    let my_app = &mut MyApp {
        buttons: vec![
            Button::new(
                Colors::ROYAL_BLUE,
                rect!(100, 100, 200, 100),
                Some(9),
                Some(Text::new("New bloc".to_string(), 20, None)),
            ),
            Button::new(Colors::GREY, rect!(550, 20, 80, 50), Some(7), None),
            Button::new(
                Colors::GREEN,
                rect!(400, 200, 100, 100),
                Some(8),
                Some(Text::new("Reset Slider 1".to_string(), 20, None)),
            ),
        ],
        sliders: vec![
            Slider::new(
                Colors::GREEN,
                rect!(500, 150, 180, 18),
                Some(4),
                SliderType::Discrete {
                    snap: 10,
                    default_value: 5,
                    display: Some(Box::new(|value| format!("R{}", 2_u32.pow(value + 1)))),
                },
            ),
            Slider::new(
                Colors::ORANGE,
                rect!(700, 80, 40, 150),
                Some(20),
                SliderType::Continuous {
                    default_value: 0.25,
                    display: Some(Box::new(|value| format!("{:.2}", value * 100.0 - 50.0))),
                },
            ),
        ],
        id_counter: 0,
        bloc_state: AppState::Idle,
        blocs: HashMap::new(),
        text: String::new(),
    };

    let mut app: App = App::init("Benday", 1200, 720, Some(60.0), true, Colors::SKY_BLUE);
    app.run(my_app);
}
