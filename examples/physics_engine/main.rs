mod constrains;
mod force_generators;
mod linear_alogithmes;
mod particle;

use constrains::{Constrain, LengthConstraint, LineConstraint, SlidingConstraint};
use force_generators::{ForceGenerator, Gravity, Spring};
use linear_alogithmes::gauss_seidel;
use nalgebra;
use nalgebra::{Point2, Vector2};
use ndarray::{Array1, Array2};
use particle::Particle;
use pg_sdl::prelude::*;
use pg_sdl::widgets::switch::Switch;
use pg_sdl::widgets::Widgets;
use sdl2::ttf::FontStyle;
use std::collections::HashMap;

/// PhysicsApp is a pyhsics engine app made to test any kind of 2D physics.
pub struct PhysicsApp {
	camera: Camera,
	background_color: Color,
	time: f64,
	original_particles: Vec<Particle>,
	particles: Vec<Particle>,
	constrains: Vec<Box<dyn Constrain>>,
	force_generators: Vec<Box<dyn ForceGenerator>>,
	last_lambda: Option<Array1<f64>>,
	mouse_spring: Spring,
	draw_forces: bool,
}

impl PhysicsApp {
	const KS: f64 = 2.0;
	const KD: f64 = 1.0;

	fn new(
		camera: Camera, background_color: Color, particles: Vec<Particle>, constrains: Vec<Box<dyn Constrain>>,
		force_generators: Vec<Box<dyn ForceGenerator>>,
	) -> Self {
		// Add a particle and a spring for the mouse
		let mut particles = particles;
		particles.insert(0, Particle::new(1.0, Point2::origin(), 0.0, Colors::GREY));
		let mouse_spring = Spring::new(0, 0, 100.0, 10.0, 0.0, 30.0, Colors::LIGHT_GREY);

		// Initialize the constrains
		let mut constrains = constrains;
		constrains.iter_mut().for_each(|constrain| constrain.init(&particles));

		Self {
			camera,
			background_color,
			time: 0.0,
			original_particles: particles.iter().map(|particle| particle.clone()).collect(),
			particles,
			constrains,
			force_generators,
			last_lambda: None,
			mouse_spring,
			draw_forces: false,
		}
	}

	fn manage_input(&mut self, input: &Input, widgets: &mut Widgets) {
		if widgets.get_button("reset").state.is_pressed() {
			self.particles = self.original_particles.iter().map(|particle| particle.clone()).collect();
			self.constrains.iter_mut().for_each(|constrain| constrain.init(&self.particles));
		}

		if input.mouse.left_button.is_pressed() {
			let mouse_position = input.mouse.position;
			for (index, particle) in self.particles.iter().enumerate() {
				if particle.collide_point(self.camera.transform.inverse() * mouse_position.cast()) {
					self.mouse_spring.set_end2_index(index);
					break;
				}
			}
		} else if input.mouse.left_button.is_released() {
			self.mouse_spring.set_end2_index(0);
		}
	}

	fn update_physics(&mut self, delta: f64) {
		// 1 - Clear forces
		self.particles.iter_mut().for_each(|particle| {
			particle.clear_force_accumulator();
		});
		// 2 - Apply the forces of the force generators
		self.mouse_spring.apply_forces(&mut self.particles);
		self.force_generators.iter_mut().for_each(|force_generator| {
			force_generator.apply_forces(&mut self.particles);
		});
		// 3 - Apply the constrains forces (or reaction forces)
		let constrain_forces = self.get_constrain_forces(1.0);
		for (particle, force) in self.particles.iter_mut().zip(constrain_forces.into_iter()) {
			particle.apply_force(force);
		}
		// 4 - Update particles
		self.particles.iter_mut().for_each(|particle| {
			particle.update(delta);
		});
	}

	fn get_constrain_forces(&mut self, threshold: f64) -> Vec<Vector2<f64>> {
		let particle_size = 2 * self.particles.len();
		let constrain_size = self.constrains.len();

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
		// Right side (B) of the equation -J. * v - J * W * f - Ks * c - Kd * c_derivative
		let b = -j_derivative.dot(&v_vector)
			- j_matrix.dot(&w_matrix).dot(&f_vector)
			- PhysicsApp::KS * &c_vector
			- PhysicsApp::KD * &c_derivative;

		let lambda = gauss_seidel(a, b, self.last_lambda.clone(), threshold);

		let reaction_vector = j_matrix.t().dot(&lambda);
		self.last_lambda = Some(lambda.into_owned());

		// let lambda = Array1::<f64>::zeros(particle_size);
		let mut reaction_forces = Vec::<Vector2<f64>>::new();
		for (index, _particle) in self.particles.iter().enumerate() {
			let reaction_force = Vector2::new(reaction_vector[2 * index] as f64, reaction_vector[2 * index + 1] as f64);
			reaction_forces.push(reaction_force);
		}

		// let j = Vec2::new(*j_matrix.get((0, 2)).unwrap() as f32, *j_matrix.get((0, 3)).unwrap() as f32);
		// let force = self.particles[1].get_force();
		// println!("force = {:?}", force);
		// let reaction_force_2 = -force.projected_onto(j);
		// println!("reaction force 1 = {:?}", reaction_forces[1]);
		// println!("reaction force 2 = {:?}", reaction_force_2);
		// reaction_forces[1] = reaction_force_2;
		reaction_forces
	}
}

impl App for PhysicsApp {
	fn update(&mut self, delta: f64, input: &Input, widgets: &mut Widgets) -> bool {
		let mut changed = false;
		self.manage_input(input, widgets);

		self.draw_forces = widgets.get::<Switch>("switch").unwrap().is_switched();

		let speed = widgets.get_slider("speed").get_value() as f64;
		self.time += delta * speed as f64;

		changed |= speed != 0.0;
		if changed {
			self.update_physics(0.02 * speed);
		}

		// Moves the selected particle to the mouse position and sets its velocity to the mouse velocity
		self.particles[0].set_position(self.camera.transform.inverse() * input.mouse.position.cast());
		let velocity = self.particles[0].get_velocity();
		let mouse_velocity = input.mouse.delta.cast() / delta;
		self.particles[0].set_velocity(velocity * 0.9 + mouse_velocity * 0.1);

		if self.mouse_spring.get_end2_index() == 0 {
			changed |= self.camera.update(input);
		}
		changed
	}

	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer) {
		self.camera.draw_grid(canvas, text_drawer, self.background_color, true, false);
		self.particles.iter().for_each(|particle| particle.draw(canvas, &self.camera));
		self.constrains.iter().for_each(|constrain| constrain.draw(canvas, &self.camera, &self.particles));
		self.force_generators
			.iter()
			.for_each(|force_generator| force_generator.draw(canvas, &self.camera, &self.particles));
		if self.draw_forces {
			self.particles.iter().for_each(|particle| particle.draw_forces(canvas, &self.camera, 0.05));
		}
		self.mouse_spring.draw(canvas, &self.camera, &self.particles);

		text_drawer.draw(
			canvas,
			point!(15, 50),
			&TextStyle::new(25, None, Color::BLACK, FontStyle::NORMAL),
			&format!("time {:.2}", self.time),
			Align::TopLeft,
		);
		/*
		let p = Vec2::new(900.0, 300.0);
		let delta = self.particles[2].get_position() - self.particles[1].get_position();
		let d = p + Vec2::new_y((150.0 - delta.length()) * 1.0);
		DrawRenderer::line(canvas, p.x as i16, p.y as i16, d.x as i16, d.y as i16, Color::BLACK).unwrap();
		*/
	}
}

fn main() {
	let resolution = Vector2::new(1200, 700);
	let background_color = Colors::SKY_BLUE;

	let camera = Camera::new(resolution, 6, 3.0, 5.0, -5000.0, 5000.0, -5000.0, 5000.0);

	let mut my_app = PhysicsApp::new(
		camera,
		background_color,
		Vec::from([
			Particle::new(1.0, Point2::new(0.0, 0.0), 25.0, Colors::RED),
			Particle::new(1.0, Point2::new(200.0, 0.0), 25.0, Colors::RED),
			// Particle::new(1.0, Vec2::new(750.0, 450.0), 25.0, Colors::RED),
		]),
		Vec::from([
			// Box::new(FixedConstraint::new(1, Colors::BLUE)) as Box<dyn Constrain>,
			// Box::new(ParabolaConstraint::new(1, Vec2::new_x(-30.0), 0.0, Colors::BLACK)) as Box<dyn Constrain>,
			Box::new(LineConstraint::new(1, Vector2::new(1.0, 0.0), Colors::BLUE)) as Box<dyn Constrain>,
			Box::new(LineConstraint::new(1, Vector2::new(0.0, 1.0), Colors::GREEN)) as Box<dyn Constrain>,
			// Box::new(LengthConstraint::new(1, 2, 10.0, Colors::BROWN)),
			// Box::new(SlidingConstraint::new(2, 3, Vec2::from_angle_deg(2.0))),
		]),
		Vec::from([
			Box::new(Gravity::new(Vector2::new(0.0, 800.0))) as Box<dyn ForceGenerator>,
			Box::new(Spring::new(1, 2, 30.0, 1.0, 150.0, 50.0, Colors::BEIGE)),
		]),
	);

	let mut app: PgSdl = PgSdl::init("Spring test", resolution.x, resolution.y, Some(60), true, background_color);
	let slider_type =
		SliderType::Continuous { default_value: 0.0, display: Some(Box::new(|value| format!("{:.2}", value))) };
	let slider = Slider::new(Colors::ORANGE, rect!(500, 50, 200, 32), 16, slider_type);
	let button =
		Button::new(Colors::LIGHT_YELLOW, rect!(750, 35, 120, 50), Some(9), TextStyle::default(), "Reset".to_string());
	let switch = Switch::new(Colors::VIOLET, Colors::DARK_VIOLET, rect!(920, 40, 25, 40), 10);
	app.add_widgets(HashMap::from([
		("reset", Box::new(button) as Box<dyn Widget>),
		("speed", Box::new(slider) as Box<dyn Widget>),
		("switch", Box::new(switch) as Box<dyn Widget>),
	]));

	app.run(&mut my_app);
}
