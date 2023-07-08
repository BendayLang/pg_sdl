#![allow(dead_code)]

use crate::particle::Particle;
use nalgebra::{Affine2, Isometry2, Matrix3, Point2, Rotation2, Similarity2, Transform2, UnitVector2, Vector2};
use pg_sdl::camera::Camera;
use pg_sdl::color::{darker, Colors};
use pg_sdl::vector2::Vector2Plus;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::{PI, TAU};

pub trait ForceGenerator {
	fn apply_forces(&self, particles: &mut Vec<Particle>);
	fn draw(&self, canvas: &mut Canvas<Window>, camera: &Camera, particles: &Vec<Particle>);
}

#[derive(Debug)]
pub struct Gravity {
	acceleration: Vector2<f64>,
}
impl Gravity {
	pub fn new(acceleration: Vector2<f64>) -> Self {
		Self { acceleration }
	}
}
impl ForceGenerator for Gravity {
	fn apply_forces(&self, particles: &mut Vec<Particle>) {
		particles.iter_mut().for_each(|particle| {
			particle.apply_force(self.acceleration * particle.get_mass());
		});
	}
	fn draw(&self, _canvas: &mut Canvas<Window>, _camera: &Camera, _particles: &Vec<Particle>) {}
}

/// A physics_engine is a force generator.
///
/// It attracts or repels 2 particles (end1 and end2), forcing them to maintain a fixed distance.
///
/// The quantities of the physics_engine are:
/// - the **rest length**
/// - the force constant **k** (N/m)
/// - the damping **b** (kg/s)
#[derive(Debug)]
pub struct Spring {
	end1_index: usize,
	end2_index: usize,
	k: f64,
	b: f64,
	rest_length: f64,
	diameter: f64,
	color: Color,
}
impl Spring {
	pub fn new(
		end1_index: usize, end2_index: usize, k: f64, b: f64, default_length: f64, diameter: f64, color: Color,
	) -> Self {
		Self { rest_length: default_length, end1_index, end2_index, k, b, diameter, color }
	}
	pub fn set_end2_index(&mut self, end2_index: usize) {
		self.end2_index = end2_index;
	}
	pub fn get_end2_index(&mut self) -> usize {
		self.end2_index // TODO <- find a better way to grab particles
	}
}
impl ForceGenerator for Spring {
	fn apply_forces(&self, particles: &mut Vec<Particle>) {
		if self.end1_index == self.end2_index {
			return;
		}

		let delta_position = particles[self.end2_index].get_position() - particles[self.end1_index].get_position();
		let delta_velocity = particles[self.end2_index].get_velocity() - particles[self.end1_index].get_velocity();
		let direction = delta_position.normalize();

		let force = -self.k * (delta_position.norm() - self.rest_length);
		let damping = -self.b * delta_velocity.dot(&direction);
		let force = direction * (force + damping);

		particles[self.end1_index].apply_force(-force);
		particles[self.end2_index].apply_force(force);
	}

	fn draw(&self, canvas: &mut Canvas<Window>, camera: &Camera, particles: &Vec<Particle>) {
		if self.end1_index == self.end2_index {
			return;
		}
		let start_position = particles[self.end1_index].get_position();
		let end_position = particles[self.end2_index].get_position();
		let delta = end_position - start_position;

		let n = 5; // Number of turns
		let d = 0.1; // Small diameter in proportion to large diameter
		let m = 1.4; // Width of the end parts in proportion to small diameter
		let f = 0.5; // Length of the end parts in proportion to large diameter

		let non_uniform_scaling = |x_scaling: f64, y_scaling: f64| {
			Affine2::from_matrix_unchecked(Matrix3::new(x_scaling, 0.0, 0.0, 0.0, y_scaling, 0.0, 0.0, 0.0, 1.0))
		};

		if delta.norm() / self.diameter <= 2.0 * f {
			let transform = Isometry2::new(start_position.coords, delta.get_angle())
				* non_uniform_scaling(self.diameter * f, self.diameter);

			let o = delta.norm() / self.diameter - 2.0 * f + 1.0;
			let mut poly = Vec::from([
				Point2::new(o - d * m / f, -d * m),
				Point2::new(o - d * m / f, -0.5),
				Point2::new(o + d * m / f, -0.5),
				Point2::new(o + d * m / f, -d * m),
			]);
			poly.extend(
				(0..=5)
					.map(|i| {
						let v = Vector2::new_unitary((i as f64 / 5.0 - 0.5) * PI);
						Point2::new(v.x / f, v.y) * d * m + Vector2::new(o * 2.0, 0.0)
					})
					.collect::<Vec<Point2<f64>>>(),
			);
			poly.extend(Vec::from([
				Point2::new(o + d * m / f, d * m),
				Point2::new(o + d * m / f, 0.5),
				Point2::new(o - d * m / f, 0.5),
				Point2::new(o - d * m / f, d * m),
			]));
			poly.extend(
				(0..=5)
					.map(|i| {
						let v = Vector2::new_unitary((i as f64 / 5.0 + 0.5) * PI);
						Point2::new(v.x / f, v.y) * d * m
					})
					.collect::<Vec<Point2<f64>>>(),
			);

			let poly = poly.iter().map(|point| transform * point).collect();
			camera.fill_polygon(canvas, self.color, &poly);
			camera.draw_polygon(canvas, Colors::BLACK, &poly);
		} else {
			let spacing = 1.0 / (2 * n + 1) as f64;
			let dl = (delta.norm() / self.diameter - 2.0) * spacing;
			let ld = d * self.diameter * (dl * dl + 1.0).sqrt();
			let darker_color = darker(self.color, 0.8);

			let start_transform = Isometry2::new(start_position.coords, delta.get_angle())
				* non_uniform_scaling(self.diameter * f, self.diameter);
			let end_transform = Isometry2::new(end_position.coords, delta.get_angle())
				* non_uniform_scaling(-self.diameter * f, self.diameter);

			let isometry = Isometry2::new(start_position.coords + start_transform * Vector2::x(), delta.get_angle());
			let scaling = non_uniform_scaling(delta.norm() - self.diameter * f * 2.0, self.diameter * 0.5);

			// Draws the back spires
			(0..=n).for_each(|i| {
				let m = (2 * i) as f64 * spacing;
				let p1 = scaling * Point2::new(m, 1.0);
				let p2 = scaling * Point2::new(m + spacing, -1.0);

				let poly = Vec::from([
					p1 + Vector2::new(-ld, 0.0),
					p1 + Vector2::new(ld, 0.0),
					p2 + Vector2::new(ld, 0.0),
					p2 + Vector2::new(-ld, 0.0),
				])
				.iter()
				.map(|point| isometry * point)
				.collect();

				camera.fill_polygon(canvas, darker_color, &poly);
				camera.draw_polygon(canvas, Colors::BLACK, &poly);
			});
			// Draws the front spires
			(0..n).for_each(|i| {
				let m = ((2 * i + 1) as f64) * spacing;
				let p1 = scaling * Point2::new(m, -1.0);
				let p2 = scaling * Point2::new(m + spacing, 1.0);

				let poly = Vec::from([
					p1 + Vector2::new(-ld, 0.0),
					p1 + Vector2::new(ld, 0.0),
					p2 + Vector2::new(ld, 0.0),
					p2 + Vector2::new(-ld, 0.0),
				])
				.iter()
				.map(|point| isometry * point)
				.collect();

				camera.fill_polygon(canvas, self.color, &poly);
				camera.draw_polygon(canvas, Colors::BLACK, &poly);
			});

			// Draws the ends
			let mut poly = Vec::from([
				Point2::new(1.0 - d * m / f, -d * m),
				Point2::new(1.0 - d * m / f, -0.5),
				Point2::new(1.0 + d * m / f, -0.5),
				Point2::new(1.0 + d * m / f, 0.5),
				Point2::new(1.0 - d * m / f, 0.5),
				Point2::new(1.0 - d * m / f, d * m),
			]);
			let poly_r: Vec<Point2<f64>> = (0..=5)
				.map(|i| {
					let v = Vector2::new_unitary((i as f64 / 5.0 + 0.5) * PI);
					Point2::new(v.x / f, v.y) * d * m
				})
				.collect();
			poly.extend(poly_r);

			let poly_start = poly.iter().map(|point| start_transform * point).collect();
			camera.fill_polygon(canvas, self.color, &poly_start);
			camera.draw_polygon(canvas, Colors::BLACK, &poly_start);
			let poly_end = poly.iter().map(|point| end_transform * point).collect();
			camera.fill_polygon(canvas, self.color, &poly_end);
			camera.draw_polygon(canvas, Colors::BLACK, &poly_end);
		}
	}
}

/// A motor is a force generator.
///
/// It applies a force to a particle (start) to make it rotate around another particle (end).
///
/// It is generally used in conjunction with a `Rod` to make a motorized joint.
#[derive(Debug)]
pub struct Motor {
	start: usize,
	end: usize,
	speed: f64,
	color: Color,
}
impl Motor {
	pub fn new(start: usize, end: usize, speed: f64, color: Color) -> Self {
		Self { start, end, speed, color }
	}
}
impl ForceGenerator for Motor {
	fn apply_forces(&self, particles: &mut Vec<Particle>) {
		let start_position = particles[self.start].get_position();
		let end_position = particles[self.end].get_position();
		let delta_position = end_position - start_position;

		// particles[self.end].position = start_position + delta_position.rotated(self.speed * delta);
		particles[self.end].apply_force(delta_position.perpendicular() * self.speed);
	}

	fn draw(&self, canvas: &mut Canvas<Window>, camera: &Camera, particles: &Vec<Particle>) {
		let start_position = particles[self.start].get_position();
		let end_position = particles[self.end].get_position();
		let delta_position = end_position - start_position;
		let radius = delta_position.norm();
		let angle = delta_position.get_angle().to_degrees();

		let w = 5;

		if radius as i16 - w <= 0 {
			return;
		}
		/*
				DrawRenderer::filled_circle(
					canvas,
					start_position.x as i16,
					start_position.y as i16,
					radius as i16 + w,
					self.color,
				)
				.unwrap();
		*/
		DrawRenderer::circle(
			canvas,
			start_position.x as i16,
			start_position.y as i16,
			radius as i16 - w,
			Colors::BLACK,
		)
		.unwrap();

		DrawRenderer::circle(
			canvas,
			start_position.x as i16,
			start_position.y as i16,
			radius as i16 + w,
			Colors::BLACK,
		)
		.unwrap();

		/*
		(0..4).into_iter().for_each(|i| {
			let angle = angle + i as f64 / 4.0 * 360.0;
			let t1 = start_position + Vector2::from_polar_deg(radius + w as f64 - 1.0, angle);
			let t2 = start_position + Vector2::from_polar_deg(radius + w as f64 - 2.0, angle + 45.0);
			DrawRenderer::thick_line(
				canvas,
				start_position.x as i16,
				start_position.y as i16,
				t1.x as i16,
				t1.y as i16,
				2 * w as u8,
				Colors::BLACK,
			)
			.unwrap();
			DrawRenderer::filled_pie(
				canvas,
				t1.x as i16,
				t1.y as i16,
				w,
				angle as i16 - 90,
				angle as i16 + 90,
				Colors::BLACK,
			)
			.unwrap();
			DrawRenderer::filled_pie(
				canvas,
				t2.x as i16,
				t2.y as i16,
				w * 2,
				angle as i16 - 45,
				angle as i16 + 135,
				self.color,
			)
			.unwrap();
			DrawRenderer::pie(
				canvas,
				t2.x as i16,
				t2.y as i16,
				w * 2,
				angle as i16 - 45,
				angle as i16 + 135,
				Colors::BLACK,
			)
			.unwrap();
		});
		 */
	}
}
