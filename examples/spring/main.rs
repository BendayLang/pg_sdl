mod constrains;
mod force_generators;
mod particle;

use crate::force_generators::Gravity;
use constrains::Constrain;
use constrains::Rod;
use force_generators::{ForceGenerator, Motor, Spring};
use particle::Particle;
use pg_sdl::get_slider;
use pg_sdl::prelude::*;
use pg_sdl::vector2::Vec2;
use pg_sdl::widgets::Widgets;
use std::collections::HashMap;

/// PhysicsApp is a pyhsics engine app made to test any kind of 2D physics.
pub struct PhysicsApp {
    particles: Vec<Particle>,
    constrains: Vec<Box<dyn Constrain>>,
    force_generators: Vec<Box<dyn ForceGenerator>>,
    selected_particle: Option<usize>,
}

impl App for PhysicsApp {
    fn update(&mut self, delta: f32, input: &Input, widgets: &mut Widgets) -> bool {
        if widgets.get_mut_button("play").state.is_pressed() {
            if get_slider!(widgets, "speed").get_value() == 0.0 {
                widgets.get_mut::<Slider>("speed").unwrap().set_value(1.0);
                widgets
                    .get_mut::<Button>("play")
                    .unwrap()
                    .set_text("Stop".to_string());
            } else {
                widgets.get_mut::<Slider>("speed").unwrap().set_value(0.0);
                widgets
                    .get_mut::<Button>("play")
                    .unwrap()
                    .set_text("Start".to_string());
            }
        }

        if input.mouse.left_button.is_pressed() {
            let mouse_position = Vec2::from(input.mouse.position);
            self.particles
                .iter()
                .enumerate()
                .for_each(|(index, particle)| {
                    // TODO replace by take_while()
                    if particle.collide_point(mouse_position) {
                        self.selected_particle = Some(index);
                    }
                });
        } else if input.mouse.left_button.is_released() {
            self.selected_particle = None;
        }

        // The physics happens here

        // 1 - Clear forces
        self.particles.iter_mut().for_each(|particle| {
            particle.clear_force_accumulator();
        });
        // 2 - Apply forces
        self.force_generators
            .iter_mut()
            .for_each(|force_generator| {
                force_generator.apply_forces(&mut self.particles);
            });
        // 3 - Apply constrains
        // TODO apply constrains
        // 4 - Update particles
        let delta = delta * widgets.get_mut::<Slider>("speed").unwrap().get_value() * 10.0;
        self.particles.iter_mut().for_each(|particle| {
            particle.update(delta);
        });

        // The physics stops here

        if let Some(index) = self.selected_particle {
            self.particles[index].set_position(Vec2::from(input.mouse.position));
            self.particles[index].set_velocity(Vec2::ZERO);
        }

        let changed = get_slider!(widgets, "speed").get_value() != 0.0;
        changed
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, _text_drawer: &mut TextDrawer) {
        self.constrains
            .iter()
            .for_each(|constrain| constrain.draw(canvas, &self.particles));

        self.particles
            .iter()
            .for_each(|particle| particle.draw(canvas));
        self.force_generators
            .iter()
            .for_each(|force_generator| force_generator.draw(canvas, &self.particles));
    }
}

fn main() {
    let particles = Vec::from([
        Particle::new(Vec2::new(0.0, 0.0), 0.0, 0.0, Colors::BLACK, true),
        Particle::new(Vec2::new(600.0, 400.0), 1.0, 20.0, Colors::ORANGE, true),
        Particle::new(Vec2::new(600.0, 450.0), 1.0, 15.0, Colors::ORANGE, true),
        Particle::new(Vec2::new(600.0, 550.0), 5.0, 25.0, Colors::RED, false),
        Particle::new(Vec2::new(800.0, 200.0), 1.0, 20.0, Colors::GREEN, true),
        Particle::new(Vec2::new(800.0, 650.0), 0.5, 20.0, Colors::MAGENTA, false),
    ]);

    let my_app = &mut PhysicsApp {
        constrains: Vec::from([
            Box::new(Rod::new(3, 4, 10.0, Colors::BROWN, &particles)) as Box<dyn Constrain>,
            // Rod::new(3, 5, 50.0, 10.0, Colors::BROWN),
        ]),
        particles,
        force_generators: Vec::from([
            Box::new(Gravity::new(Vec2::new_y(9.81))) as Box<dyn ForceGenerator>,
            Box::new(Motor::new(1, 2, 0.4, Colors::LIGHT_GREY)),
            Box::new(Spring::new(0, 0, 1.0, 0.5, 0.0, 20.0, Colors::WHITE)),
            Box::new(Spring::new(2, 3, 0.5, 0.2, 150.0, 40.0, Colors::BEIGE)),
            Box::new(Spring::new(3, 5, 5.0, 2.0, 50.0, 20.0, Colors::LIGHT_GREEN)),
        ]),
        selected_particle: None,
    };

    let mut app: PgSdl = PgSdl::init("Spring test", 1200, 720, Some(60), true, Colors::SKY_BLUE);
    app.add_widgets(HashMap::from([
        (
            "play",
            Box::new(Button::new(
                Colors::ORANGE,
                rect!(300, 35, 120, 50),
                Some(9),
                Some(Text::new("Start".to_string(), 18, None)),
            )) as Box<dyn Widget>,
        ),
        (
            "speed",
            Box::new(Slider::new(
                Colors::ORANGE,
                rect!(500, 50, 200, 30),
                Some(20),
                SliderType::Continuous {
                    default_value: 0.0,
                    display: Some(Box::new(|value| format!("{:.2}", value))),
                },
            )),
        ),
    ]));
    app.run(my_app);
}
