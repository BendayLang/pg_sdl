#![allow(dead_code, unused_variables)]
mod blocs;

use crate::blocs::{draw_bloc, set_child};
use blocs::Bloc;
use pg_sdl::canvas;
use pg_sdl::prelude::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::ttf::FontStyle;
use std::collections::HashMap;

enum AppState {
    Idle,
    Selected { id: u32 },
}

pub struct MyApp {
    id_counter: u32,
    bloc_state: AppState,
    blocs: HashMap<u32, Bloc>,
}

impl App for MyApp {
    fn update(&mut self, _delta: f32, input: &Input, widgets: &mut Widgets) -> bool {
        let mut changed = false;
        match self.bloc_state {
            AppState::Idle => {
                // Add a bloc
                if widgets.get_button("Add").state.is_pressed() {
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
                    .set_position((input.mouse.position.x, input.mouse.position.y));
                // .move_by(input.mouse.delta);
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
    }
}

fn main() {
    let my_app = &mut MyApp {
        id_counter: 0,
        bloc_state: AppState::Idle,
        blocs: HashMap::new(),
    };

    let mut app = PgSdl::init("Benday", 1280, 720, Some(60), true, Color::GREY);

    app.add_widget(
        "Add",
        Box::new(Button::new(
            Colors::ROYAL_BLUE,
            rect!(100, 100, 200, 100),
            Some(9),
            TextStyle::new(20, None, Color::BLACK, FontStyle::BOLD),
            "New bloc".to_string(),
        )),
    );

    app.run(my_app);
}
