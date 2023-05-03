#![allow(dead_code, unused_variables)]

use pg_sdl::blocs::set_child;
use pg_sdl::prelude::*;
use sdl2::gfx::primitives::DrawRenderer;
use std::collections::HashMap;

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
        let mut changed = false;
        changed |= self
            .widgets()
            .iter_mut()
            .any(|widget| widget.update(&input, delta));

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
                else if input.mouse.left_button.is_pressed() {
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
                    let maybe_parent_bloc: Option<&Bloc> = self
                        .blocs
                        .values()
                        .into_iter()
                        .find(|bloc| moving_bloc.collide_bloc(bloc));

                    let collide_with: Option<u32> = {
                        let mut temp = None;
                        for (id, bloc) in &self.blocs {
                            if id == &moving_bloc_id {
                                continue;
                            }
                            if moving_bloc.collide_bloc(bloc) {
                                temp = Some(id);
                            }
                        }
                        temp
                    }
                    .copied();

                    if let Some(parent_id) = collide_with {
                        set_child(moving_bloc_id, parent_id, &mut self.blocs);
                    }

                    changed = true;
                    self.bloc_state = AppState::Idle;
                }
            }
        }

        changed
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        self.widgets()
            .iter()
            .for_each(|widget| widget.draw(canvas, text_drawer));

        let radius = 2_u32.pow(self.sliders[0].get_value() as u32 + 1);
        let rect = (200, 400, 520, 600);
        DrawRenderer::rounded_box(
            canvas,
            rect.0,
            rect.1,
            rect.2,
            rect.3,
            radius as i16,
            Colors::GREEN,
        )
        .expect("DrawRenderer failed");
        DrawRenderer::rounded_rectangle(
            canvas,
            rect.0,
            rect.1,
            rect.2,
            rect.3,
            radius as i16,
            Colors::BLACK,
        )
        .expect("DrawRenderer failed");

        let texture_creator = canvas.texture_creator();
        for (_id, bloc) in &self.blocs {
            if bloc.parent != None {
                continue;
            }
            let surface = draw_bloc(bloc, &self.blocs, canvas, text_drawer, &texture_creator);
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();
            canvas.copy(&texture, None, Some(bloc.get_rect())).unwrap();
        }

        // text of blocs
        let text = format!(
            "{}",
            self.blocs
                .iter()
                .map(|(id, bloc)| format!(" {}: {} ", id, bloc))
                .collect::<Vec<String>>()
                .join("\n")
        );
        // text_drawer.draw(
        //     canvas,
        //     &Text::new(text, 12.0),
        //     point!(130.0, 550.0),
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
            Button::new(Colors::GREY, rect!(550, 20, 80, 50), None, None),
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
                rect!(500, 150, 150, 18),
                Some(4),
                SliderType::Discrete {
                    snap: 6,
                    default_value: 3,
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
    };

    let mut app: App = App::init("Benday", 1200, 720, Some(60.0), true, Colors::SKY_BLUE);
    app.run(my_app);
}
