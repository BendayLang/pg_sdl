mod physics_objects;

use pg_sdl::prelude::*;
use pg_sdl::vector2::Vec2;
use physics_objects::{apply_gravity, Mass, Motor, Rod, Spring};

/// My app is the starting point of the application.
pub struct MyApp {
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
    masses: Vec<Mass>,
    rods: Vec<Rod>,
    springs: Vec<Spring>,
    motors: Vec<Motor>,
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
        self.widgets()
            .iter_mut()
            .any(|widget| widget.update(&input, delta));

        if self.buttons[0].state.is_pressed() {
            if self.sliders[0].get_value() == 0.0 {
                self.sliders[0].set_value(1.0);
                self.buttons[0].set_text("Stop".to_string());
            } else {
                self.sliders[0].set_value(0.0);
                self.buttons[0].set_text("Start".to_string());
            }
        }

        if input.mouse.left_button.is_pressed() {
            let mouse_position = Vec2::from(input.mouse.position);
            self.masses.iter().enumerate().for_each(|(index, mass)| {
                // TODO replace by take_while()
                if mass.collide_point(mouse_position) {
                    self.springs[0].change_end(index);
                }
            });
        } else if input.mouse.left_button.is_released() {
            self.springs[0].change_end(0);
        }

        apply_gravity(&mut self.masses);
        self.springs.iter_mut().for_each(|spring| {
            spring.apply_force(&mut self.masses);
        });

        let delta = delta * self.sliders[0].get_value() * 10.0;

        self.masses.iter_mut().for_each(|mass| mass.update(delta));
        self.motors
            .iter()
            .for_each(|motor| motor.update(delta, &mut self.masses));

        self.masses[0].position = Vec2::from(input.mouse.position);

        // set mass 3 for constraining rod to its initial length
        let delta_length = self.masses[3].position - self.masses[4].position;
        self.masses[3].position =
            self.masses[4].position + delta_length.normalized() * self.rods[0].length;

        let delta_velocity = self.masses[3].velocity - self.masses[4].velocity;
        let v1 = delta_velocity.dot(delta_length.perpendicular());
        self.masses[3].velocity = self.masses[4].velocity + delta_length.perpendicular() * v1;

        true
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        self.widgets()
            .iter()
            .for_each(|widget| widget.draw(canvas, text_drawer));

        self.motors
            .iter()
            .for_each(|motor| motor.draw(canvas, &self.masses));
        self.masses.iter().for_each(|mass| mass.draw(canvas));
        self.rods
            .iter()
            .for_each(|rod| rod.draw(canvas, &self.masses));
        self.springs
            .iter()
            .for_each(|spring| spring.draw(canvas, &self.masses));
    }
}

fn main() {
    let my_app = &mut MyApp {
        buttons: vec![Button::new(
            Colors::ORANGE,
            rect!(300, 35, 120, 50),
            Some(9),
            Some(Text::new("Start".to_string(), 16, None)),
        )],
        sliders: vec![Slider::new(
            Colors::ORANGE,
            rect!(500, 50, 200, 30),
            Some(20),
            SliderType::Continuous {
                default_value: 0.0,
                display: Some(Box::new(|value| format!("{:.2}", value))),
            },
        )],

        masses: Vec::from([
            Mass::new(Vec2::new(0.0, 0.0), 0.0, 0.0, Colors::BLACK, true),
            Mass::new(Vec2::new(600.0, 400.0), 1.0, 20.0, Colors::ORANGE, true),
            Mass::new(Vec2::new(600.0, 450.0), 1.0, 15.0, Colors::ORANGE, true),
            Mass::new(Vec2::new(600.0, 550.0), 5.0, 25.0, Colors::RED, false),
            Mass::new(Vec2::new(800.0, 200.0), 1.0, 20.0, Colors::GREEN, true),
        ]),
        rods: Vec::from([Rod::new(3, 4, 250.0, 10.0, Colors::YELLOW)]),
        springs: Vec::from([
            Spring::new(0, 0, 1.0, 0.5, 0.0, 20.0, Colors::WHITE),
            Spring::new(2, 3, 0.5, 0.2, 150.0, 40.0, Colors::BEIGE),
        ]),
        motors: Vec::from([Motor::new(1, 2, 0.4, Colors::LIGHT_GREY)]),
    };

    let mut app: App = App::init("Spring test", 1200, 720, Some(90), true, Colors::SKY_BLUE);
    app.run(my_app);
}
