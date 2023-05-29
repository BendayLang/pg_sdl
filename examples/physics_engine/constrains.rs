#![allow(dead_code)]

use crate::Particle;
use pg_sdl::color::Colors;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Returns the t value of the closest point on a curve to a given point using gradient descent.
///
/// The curve is defined by a function that takes a parameter t and returns a point.
pub fn nearest_point_on_curve(point: Vec2, curve: Box<dyn Fn(f32) -> Vec2>) -> f32 {
	let function = |t: f32| ((*curve)(t) - point).length();
	let mut x = 0.0; // initial input value
	let learning_rate = 0.002; // hyperparameter controlling step size

	for _ in 0..1000 {
		// maximum number of iterations
		let gradient = (function(x + 0.0001) - function(x)) / 0.0001; // approximate gradient
		x -= learning_rate * gradient; // update input value
	}

	x // return input value at minimum
}

pub trait Constrain {
	/// Initialize the constraint (for example, compute the length of a rod)
	fn init(&mut self, particles: &Vec<Particle>);
	/// The constraint function is such that it returns 0 when the constraint is respected.
	///
	/// For a curve, let's say d is the vector from the particle to the closest point on the curve.
	///
	/// Here, the constraint function is the length of d.
	fn constrain_function(&self, particles: &Vec<Particle>) -> f32;
	/// The derivative of the constraint function is such that it returns 0 when the constraint is respected.
	///
	/// For a curve, let's say d is the vector from the particle to the closest point on the curve.
	///
	/// Here, the derivative of the constraint function is the speed of the particle along d.
	fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32;
	/// The jacobian blocs of the constraint function (dC/dx)
	///
	/// For a curve, let's say d is the vector from the particle to the closest point on the curve.
	///
	/// Here, the jacobian blocs of the constraint function are the components of d (d.x, d.y).
	fn jacobian_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)>;
	/// The jacobian blocs of the constraint derivative function (dC'/dx)
	///
	/// For a curve, let's say d is the vector from the particle to the closest point on the curve.
	///
	/// Here, the derivative of the jacobian blocs of the constraint function are null.
	fn jacobian_derivative_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)>;
	fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>);
}

/// A rod is a constraint.
///
/// It maintains 2 particles at a fixed distance (end1 and end2).
///
/// Its length is determined by the particles' positions at the time of creation.
pub struct LengthConstraint {
	end1_index: usize,
	end2_index: usize,
	length: f32,
	diameter: f32,
	color: Color,
}
impl LengthConstraint {
	pub fn new(end1_index: usize, end2_index: usize, diameter: f32, color: Color) -> Self {
		Self { end1_index, end2_index, length: 0.0, diameter, color }
	}
}
impl Constrain for LengthConstraint {
	fn init(&mut self, particles: &Vec<Particle>) {
		self.length = (particles[self.end2_index].get_position() - particles[self.end1_index].get_position()).length();
	}

	fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
		let delta = particles[self.end2_index].get_position() - particles[self.end1_index].get_position();
		delta.length_squared() - self.length.powf(2.0)
	}
	fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
		let delta = particles[self.end2_index].get_position() - particles[self.end1_index].get_position();
		let delta_velocity = particles[self.end2_index].get_velocity() - particles[self.end1_index].get_velocity();
		delta.dot(delta_velocity)
	}
	fn jacobian_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		let delta = particles[self.end2_index].get_position() - particles[self.end1_index].get_position();
		Vec::from([
			(self.end1_index * 2, -delta.x),
			(self.end1_index * 2 + 1, -delta.y),
			(self.end2_index * 2, delta.x),
			(self.end2_index * 2 + 1, delta.y),
		])
	}
	fn jacobian_derivative_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		let delta_velocity = particles[self.end2_index].get_velocity() - particles[self.end1_index].get_velocity();
		Vec::from([
			(self.end1_index * 2, -delta_velocity.x),
			(self.end1_index * 2 + 1, -delta_velocity.y),
			(self.end2_index * 2, delta_velocity.x),
			(self.end2_index * 2 + 1, delta_velocity.y),
		])
	}

	fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {
		let start_position = particles[self.end1_index].get_position();
		let end_position = particles[self.end2_index].get_position();
		let x_dir = end_position - start_position;
		let y_dir = x_dir.perpendicular() * self.diameter / 2.0;
		let start1 = start_position + y_dir;
		let start2 = start_position - y_dir;
		let end1 = end_position + y_dir;
		let end2 = end_position - y_dir;
		// draw circles at the ends
		DrawRenderer::filled_circle(
			canvas,
			start_position.x as i16,
			start_position.y as i16,
			self.diameter as i16 / 2,
			self.color,
		)
		.unwrap();
		DrawRenderer::circle(
			canvas,
			start_position.x as i16,
			start_position.y as i16,
			self.diameter as i16 / 2,
			Colors::BLACK,
		)
		.unwrap();
		DrawRenderer::filled_circle(
			canvas,
			end_position.x as i16,
			end_position.y as i16,
			self.diameter as i16 / 2,
			self.color,
		)
		.unwrap();
		DrawRenderer::circle(
			canvas,
			end_position.x as i16,
			end_position.y as i16,
			self.diameter as i16 / 2,
			Colors::BLACK,
		)
		.unwrap();
		// draw thick lines between the ends
		DrawRenderer::circle(canvas, start_position.x as i16, start_position.y as i16, self.length as i16, self.color)
			.unwrap();
		DrawRenderer::thick_line(
			canvas,
			start_position.x as i16,
			start_position.y as i16,
			end_position.x as i16,
			end_position.y as i16,
			self.diameter as u8,
			self.color,
		)
		.unwrap();
		DrawRenderer::line(canvas, start1.x as i16, start1.y as i16, end1.x as i16, end1.y as i16, Colors::BLACK)
			.unwrap();
		DrawRenderer::line(canvas, start2.x as i16, start2.y as i16, end2.x as i16, end2.y as i16, Colors::BLACK)
			.unwrap();
	}
}

/// A fixed constraint maintains a particle at a fixed position.
pub struct FixedConstraint {
	particle_index: usize,
	position: Vec2,
	color: Color,
}
impl FixedConstraint {
	pub fn new(particle: usize, color: Color) -> Self {
		Self { particle_index: particle, position: Vec2::ZERO, color }
	}
}
impl Constrain for FixedConstraint {
	fn init(&mut self, particles: &Vec<Particle>) {
		self.position = particles[self.particle_index].get_position();
	}

	fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
		let delta = particles[self.particle_index].get_position() - self.position;
		delta.length_squared() / 2.0
	}
	fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
		let delta = particles[self.particle_index].get_position() - self.position;
		let velocity = particles[self.particle_index].get_velocity();
		delta.length() * velocity.length()
	}
	fn jacobian_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		let delta = particles[self.particle_index].get_position() - self.position;
		Vec::from([(self.particle_index * 2, delta.x), (self.particle_index * 2 + 1, delta.y)])
	}
	fn jacobian_derivative_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		// Vec::new()
		let velocity = particles[self.particle_index].get_velocity();
		Vec::from([(self.particle_index * 2, velocity.x), (self.particle_index * 2 + 1, velocity.y)])
	}

	fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {
		let position = particles[self.particle_index].get_position();
		DrawRenderer::line(
			canvas,
			position.x as i16,
			position.y as i16,
			self.position.x as i16,
			self.position.y as i16,
			self.color,
		)
		.unwrap();
		// Draw the ground symbol at the fixed position.
		// TODO: draw the ground symbol
	}
}

/// A line constraint maintains a particle on a line.
///
/// The line is defined by a point and a director vector (length does not matter).
pub struct LineConstraint {
	particle_index: usize,
	point: Vec2,
	director: Vec2,
	color: Color,
}
impl LineConstraint {
	pub fn new(particle: usize, director: Vec2, color: Color) -> Self {
		Self { particle_index: particle, point: Vec2::ZERO, director, color }
	}
}
impl Constrain for LineConstraint {
	fn init(&mut self, particles: &Vec<Particle>) {
		self.point = particles[self.particle_index].get_position();
	}

	fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
		let position = particles[self.particle_index].get_position();
		let delta = position - self.point;
		delta.dot(self.director.perpendicular())
	}
	fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
		let velocity = particles[self.particle_index].get_velocity();
		velocity.dot(self.director.perpendicular())
	}
	fn jacobian_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		let jacobian = self.director.perpendicular();
		Vec::from([(self.particle_index * 2, jacobian.x), (self.particle_index * 2 + 1, jacobian.y)])
	}
	fn jacobian_derivative_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		Vec::new()
	}

	fn draw(&self, canvas: &mut Canvas<Window>, _particles: &Vec<Particle>) {
		let start = self.point - self.director * 1000.0;
		let end = self.point + self.director * 1000.0;
		DrawRenderer::line(canvas, start.x as i16, start.y as i16, end.x as i16, end.y as i16, self.color).unwrap();
	}
}

/// A parabola constraint maintains a particle on a parabola.
///
/// The parabola is defined by a point, a director vector (direction gives the x axis, length scales the curvature)
/// and a x0 value (the minimum x value of the parabola).
pub struct ParabolaConstraint {
	particle_index: usize,
	point: Vec2,
	director: Vec2,
	x0: f32,
	color: Color,
}
impl ParabolaConstraint {
	pub fn new(particle: usize, director: Vec2, x0: f32, color: Color) -> Self {
		Self { particle_index: particle, point: Vec2::ZERO, director, x0, color }
	}
	pub fn parabola_function(&self) -> Box<dyn Fn(f32) -> Vec2> {
		let point = self.point;
		let director = self.director;
		let x0 = self.x0;
		Box::new(move |t: f32| -> Vec2 { point + t * Vec2::new(1.0, t - 2.0 * x0).rotation_scale_transform(director) })
	}
	pub fn nearest_point_on_parabola(&self, point: Vec2) -> Vec2 {
		let t = nearest_point_on_curve(point, self.parabola_function());
		(*self.parabola_function())(t)
	}
}
impl Constrain for ParabolaConstraint {
	fn init(&mut self, particles: &Vec<Particle>) {
		self.point = particles[self.particle_index].get_position();
	}

	fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
		let position = particles[self.particle_index].get_position();
		let on_curve = self.nearest_point_on_parabola(position);
		let delta = on_curve - position;
		delta.length_squared() / 2.0
	}

	fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
		let velocity = particles[self.particle_index].get_velocity();
		let position = particles[self.particle_index].get_position();
		let on_curve = self.nearest_point_on_parabola(position);
		let delta = on_curve - position;
		velocity.dot(delta)
	}

	fn jacobian_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		let position = particles[self.particle_index].get_position();
		let on_curve = self.nearest_point_on_parabola(position);
		let delta = on_curve - position;
		Vec::from([(self.particle_index * 2, delta.x), (self.particle_index * 2 + 1, delta.y)])
	}

	fn jacobian_derivative_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		// let velocity = particles[self.particle_index].get_velocity();
		// Vec::from([(self.particle_index * 2, velocity.x), (self.particle_index * 2 + 1, velocity.y)])
		Vec::new()
	}

	fn draw(&self, canvas: &mut Canvas<Window>, _particles: &Vec<Particle>) {
		let function = self.parabola_function();
		let n = 30;
		let mut point1 = (*function)(-n as f32);
		let mut point2 = point1;
		((-n + 1)..=n).for_each(|i| {
			point2 = point1;
			point1 = (*function)(i as f32);
			DrawRenderer::line(canvas, point1.x as i16, point1.y as i16, point2.x as i16, point2.y as i16, self.color)
				.unwrap();
		});
	}
}

/// A sliding constraint forces a particle to slide along a line defined by another particle and a director vector.
///
/// It has tree parameters:
/// * `particle_index`: the index of the particle to constrain
/// * `anchor_index`: the index of the particle that defines the line
/// * `director`: the director vector of the line
pub struct SlidingConstraint {
	end1_index: usize,
	end2_index: usize,
	offset: f32,
	director: Vec2,
}
impl SlidingConstraint {
	pub fn new(particle_index: usize, anchor_index: usize, director: Vec2) -> Self {
		Self { end1_index: particle_index, end2_index: anchor_index, offset: 0.0, director }
	}
}
impl Constrain for SlidingConstraint {
	fn init(&mut self, particles: &Vec<Particle>) {
		let end1_position = particles[self.end1_index].get_position();
		let end2_position = particles[self.end2_index].get_position();
		let delta = end2_position - end1_position;
		self.offset = delta.dot(self.director.perpendicular());
	}

	fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
		let end1_position = particles[self.end1_index].get_position();
		let end2_position = particles[self.end2_index].get_position();
		let delta = end2_position - end1_position;
		delta.dot(self.director.perpendicular()) - self.offset
	}
	fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
		let end1_velocity = particles[self.end1_index].get_velocity();
		let end2_velocity = particles[self.end2_index].get_velocity();
		let delta_velocity = end2_velocity - end1_velocity;
		delta_velocity.dot(self.director.perpendicular())
	}
	fn jacobian_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		let jacobian = self.director.perpendicular();
		Vec::from([
			(self.end1_index * 2, -jacobian.x),
			(self.end1_index * 2 + 1, -jacobian.y),
			(self.end2_index * 2, jacobian.x),
			(self.end2_index * 2 + 1, jacobian.y),
		])
	}
	fn jacobian_derivative_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
		Vec::new()
	}

	fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {
		let end1_position = particles[self.end1_index].get_position();
		let end2_position = particles[self.end2_index].get_position();
		let middle_position = (end1_position + end2_position) * 0.5;
		let minus_end = middle_position - self.director * 1000.0;
		let plus_end = middle_position + self.director * 1000.0;
		DrawRenderer::line(
			canvas,
			minus_end.x as i16,
			minus_end.y as i16,
			plus_end.x as i16,
			plus_end.y as i16,
			Colors::RED,
		)
		.unwrap();
	}
}
