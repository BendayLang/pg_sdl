mod constrains;
mod force_generators;
mod particle;

use as_any::AsAny;
use constrains::{Constrain, FixedConstraint, LengthConstraint};
use force_generators::{ForceGenerator, Gravity, Motor, Spring};
use iterative_methods::conjugate_gradient::ConjugateGradient;
use iterative_methods::utils::LinearSystem;
use itertools::Itertools;
use ndarray::{Array1, Array2};
use particle::Particle;
use pg_sdl::prelude::*;
use pg_sdl::vector2::Vec2;
use pg_sdl::widgets::Widgets;
use sdl2::keyboard::Keycode::P;
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
        self.manage_input(input, widgets);
        self.update_physics(delta * widgets.get_mut::<Slider>("speed").unwrap().get_value() * 10.0);

        let changed = widgets.get_slider("speed").get_value() != 0.0;
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
        self.particles
            .iter()
            .for_each(|particle| particle.draw_forces(canvas, 1.0));
    }
}
impl PhysicsApp {
    const KS: f64 = 0.0;
    const KD: f64 = 0.0;

    fn manage_input(&mut self, input: &Input, widgets: &mut Widgets) {
        if widgets.get_button("play").state.is_pressed() {
            if widgets.get_slider("speed").get_value() == 0.0 {
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
            let mut min_distance = f32::MAX;
            let mut min_index = None;
            for (index, particle) in self.particles.iter().enumerate() {
                let distance = (mouse_position - particle.get_position()).length();
                if distance < min_distance {
                    min_distance = distance;
                    min_index = Some(index);
                }
            }
            if min_distance < 40.0 {
                self.selected_particle = min_index;
            }
        } else if input.mouse.left_button.is_released() {
            self.selected_particle = None;
        }

        // Moves the selected particle to the mouse position and sets its velocity to zero
        if let Some(index) = self.selected_particle {
            self.particles[index].set_position(Vec2::from(input.mouse.position));
            self.particles[index].set_velocity(Vec2::ZERO);
        }
    }

    fn update_physics(&mut self, delta: f32) {
        // 1 - Clear forces
        self.particles.iter_mut().for_each(|particle| {
            particle.clear_force_accumulator();
        });
        // 2 - Apply the forces of the force generators
        self.force_generators
            .iter_mut()
            .for_each(|force_generator| {
                force_generator.apply_forces(&mut self.particles);
            });
        // 3 - Apply the constrains forces (or reaction forces)
        let constrain_forces = self.get_constrain_forces();
        for (particle, force) in self.particles.iter_mut().zip(constrain_forces.into_iter()) {
            particle.apply_force(force);
        }
        // 4 - Update particles
        self.particles.iter_mut().for_each(|particle| {
            particle.update(delta);
        });
    }

    fn get_constrain_forces(&self) -> Vec<Vec2> {
        let particle_size = 2 * self.particles.len();
        let constrain_size = self.constrains.len();
        /*
        // State vector q of the position of the particles [p1x, p1y, p2x, p2y, ...]
        let mut q_vector = Array2::<f32>::zeros((particle_size, 1));
        for (index, particle) in self.particles.iter().enumerate() {
            Vec2 {
                x: q_vector[[2 * index, 0]],
                y: q_vector[[2 * index + 1, 0]],
            } = particle.get_position()
        }
        */

        // State vector v of the velocity of the particles [v1x, v1y, v2x, v2y, ...]
        let mut v_vector = Array1::<f64>::zeros(particle_size);
        for (index, particle) in self.particles.iter().enumerate() {
            v_vector[2 * index] = particle.get_velocity().x as f64;
            v_vector[2 * index + 1] = particle.get_velocity().y as f64;
        }
        // Matrix w (w = 1/m * I) if the identity matrix times one over the masses
        // [[1 / m1, 0     , 0     , ...],
        //  [0     , 1 / m1, 0     , ...],
        //  [0     , 0     , 1 / m2, ...],
        //  [...   , ...   , ...   , ...]]
        let mut w_matrix = Array2::<f64>::zeros((particle_size, particle_size));
        for (index, particle) in self.particles.iter().enumerate() {
            let w = 1.0 / particle.get_mass() as f64;
            w_matrix[[2 * index, 2 * index]] = w;
            w_matrix[[2 * index + 1, 2 * index + 1]] = w;
        }
        // Vector f (Q) of all the forces
        let mut f_vector = Array1::<f64>::zeros(particle_size);
        for (index, particle) in self.particles.iter().enumerate() {
            f_vector[2 * index] = particle.get_force().x as f64;
            f_vector[2 * index + 1] = particle.get_force().y as f64;
        }
        // Vector c of constrains {constrain_size}
        let mut c_vector = Array1::<f64>::zeros(constrain_size);
        for (index, constrain) in self.constrains.iter().enumerate() {
            c_vector[index] = constrain.constrain_function(&self.particles) as f64;
        }

        // Vector c_derivative of constrains {constrain_size}
        let mut c_derivative = Array1::<f64>::zeros(constrain_size);
        for (index, constrain) in self.constrains.iter().enumerate() {
            c_derivative[index] = constrain.constrain_derivative(&self.particles) as f64;
        }
        // Matrix J is the jacobian of the constrains (dc/dq)
        // [[dc1/dx1, dc1/dy1, dc1/dx2, dc1/dy2, ...],
        //  [dc2/dx1, dc2/dy1, dc2/dx2, dc2/dy2, ...],
        //  [...    ,...     ,...     ,...     , ...]]
        let mut j_matrix = Array2::<f64>::zeros((constrain_size, particle_size));
        for (constrain_index, constrain) in self.constrains.iter().enumerate() {
            let j = constrain.jacobian_blocs(&self.particles);
            for (particle_index, jacobian_bloc) in j.iter() {
                j_matrix[[constrain_index, *particle_index]] = *jacobian_bloc as f64;
            }
        }
        // Derivative of the jacobian
        let mut j_derivative = Array2::<f64>::zeros((constrain_size, particle_size));
        for (constrain_index, constrain) in self.constrains.iter().enumerate() {
            let j = constrain.jacobian_derivative_blocs(&self.particles);
            for (particle_index, jacobian_bloc) in j.iter() {
                j_derivative[[constrain_index, *particle_index]] = *jacobian_bloc as f64;
            }
        }
        // Left side (A) of the equation J * W * Jt
        let a = j_matrix.dot(&w_matrix).dot(&j_matrix.t());
        // Right side (B) of the equation -J. * v - J * W * f
        println!("constrains : {}", PhysicsApp::KS * &c_vector);
        println!("derivative : {}", PhysicsApp::KD * &c_derivative);
        let b = -j_derivative.dot(&v_vector)
            - j_matrix.dot(&w_matrix).dot(&f_vector)
            - PhysicsApp::KS * &c_vector
            - PhysicsApp::KD * &c_derivative;

        let a_clone = a.clone();
        let b_clone = b.clone();

        // using iterative_method conjugate_gradient
        let linear_system = LinearSystem {
            a: a.into_shared(),
            b: b.into_shared(),
            x0: None,
        };
        let mut algorithm = ConjugateGradient::for_problem(&linear_system);
        // algorithm.set_max_iters(1000);
        let lambda = algorithm.solution;

        let mut lambda = Array1::<f64>::zeros(constrain_size);
        lambda[0] = if a_clone[[0, 0]] == 0.0 {
            0.0
        } else {
            b_clone[0] / a_clone[[0, 0]]
        };
        println!("{} * {} = {}", &a_clone, &lambda, &b_clone);
        // panic!();

        let reaction_vector = j_matrix.t().dot(&lambda);
        println!("reaction_vector : {}", reaction_vector);

        // let lambda = Array1::<f64>::zeros(particle_size);
        let mut reaction_forces = Vec::<Vec2>::new();
        for (index, _particle) in self.particles.iter().enumerate() {
            let mut reaction_force = Vec2::new(
                reaction_vector[2 * index] as f32,
                reaction_vector[2 * index + 1] as f32,
            );
            reaction_forces.push(reaction_force);
        }
        reaction_forces
    }
}

fn main() {
    let particles = Vec::from([
        Particle::new(1.0, Vec2::new(600.0, 300.0), 25.0, Colors::RED),
        Particle::new(1.0, Vec2::new(600.0, 400.0), 25.0, Colors::ORANGE),
    ]);

    let my_app = &mut PhysicsApp {
        constrains: Vec::from([
            Box::new(FixedConstraint::new(0, Colors::BLUE, &particles)) as Box<dyn Constrain>,
            // Box::new(LengthConstraint::new(0, 1, 10.0, Colors::BROWN, &particles)),
        ]),
        particles,
        force_generators: Vec::from([
            Box::new(Gravity::new(Vec2::new_y(0.0))) as Box<dyn ForceGenerator>,
            Box::new(Spring::new(0, 1, 0.5, 0.2, 150.0, 40.0, Colors::BEIGE)),
            // Box::new(Motor::new(0, 1, 0.4, Colors::LIGHT_GREY)),
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
                TextStyle::default(),
                "Start".to_string(),
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
