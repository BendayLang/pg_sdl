use crate::color::{darker, Colors};
use crate::style::{Align, HAlign, VAlign};
use crate::text::{TextDrawer, TextStyle};
use crate::{input::Input, point};
use nalgebra::{Point2, Scale2, Similarity2, Translation2, Vector2};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::ttf::FontStyle;
use sdl2::video::Window;
use std::ops::{Neg, Not, Rem};

pub struct Camera {
	resolution: Vector2<u32>,
	pub transform: Similarity2<f64>,
}

impl Camera {
	const SCALING_FACTOR: f64 = 1.1892071150027210667174999705605; // f64::powf(2.0, 1.0 / 4.0);

	pub fn new(resolution: Vector2<u32>) -> Self {
		Camera { resolution, transform: Similarity2::identity() }
	}

	pub fn update(&mut self, input: &Input) -> bool {
		let mut changed = false;

		if input.mouse.left_button.is_down() {
			let delta = Vector2::new(input.mouse.delta.x as f64, input.mouse.delta.y as f64);
			if delta != Vector2::zeros() {
				let translation = Translation2::new(delta.x, delta.y);
				self.transform.append_translation_mut(&translation);
				changed = true;
			}
		}

		let scroll = input.mouse.wheel;
		if scroll != 0 {
			let scaling = Self::SCALING_FACTOR.powf(scroll as f64);
			let center = Point2::from(Vector2::new(input.mouse.position.x as f64, input.mouse.position.y as f64));

			let translation = Translation2::from((1.0 / scaling - 1.0) * center.coords);
			self.transform.append_translation_mut(&translation);
			self.transform.append_scaling_mut(scaling);
			changed = true;
		}

		changed
	}

	fn resize(&mut self, new_resolution: Vector2<u32>) {
		// self.move((self.resolution - new_size) / self.scale / 2);
		self.resolution = new_resolution;
	}

	/// Draws a vertical line as seen by the camera
	pub fn draw_vline(&self, canvas: &mut Canvas<Window>, color: Color, x: f64) {
		let x = self.transform.scaling() * x + self.transform.isometry.translation.x;
		DrawRenderer::vline(canvas, x as i16, 0, self.resolution.y as i16 - 1, color).unwrap();
	}
	/// Draws a horizontal line as seen by the camera
	pub fn draw_hline(&self, canvas: &mut Canvas<Window>, color: Color, y: f64) {
		let y = self.transform.scaling() * y + self.transform.isometry.translation.y;
		DrawRenderer::hline(canvas, 0, self.resolution.x as i16 - 1, y as i16, color).unwrap();
	}

	/// Draws the contour of a rectangle as seen by the camera
	pub fn draw_rectangle(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>) {
		let position = self.transform * position;
		let size = self.transform * size;
		let rect = Rect::new(position.x as i32, position.y as i32, size.x as u32, size.y as u32);
		canvas.set_draw_color(color);
		canvas.draw_rect(rect).unwrap();
	}
	/// Draws a filled rectangle as seen by the camera
	pub fn fill_rectangle(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>) {
		let position = self.transform * position;
		let size = self.transform * size;
		let rect = Rect::new(position.x as i32, position.y as i32, size.x as u32, size.y as u32);
		canvas.set_draw_color(color);
		canvas.fill_rect(rect).unwrap();
	}

	/// Draws the contour of an ellipse as seen by the camera
	pub fn draw_ellipse(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>) {
		let position = self.transform * position;
		let size = self.transform * size;
		DrawRenderer::aa_ellipse(canvas, position.x as i16, position.y as i16, size.x as i16, size.y as i16, color)
			.unwrap();
	}
	/// Draws a filled ellipse as seen by the camera
	pub fn fill_ellipse(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>) {
		let position = self.transform * position;
		let size = self.transform * size;
		DrawRenderer::filled_ellipse(canvas, position.x as i16, position.y as i16, size.x as i16, size.y as i16, color)
			.unwrap();
	}

	/// Draws the contour of a circle as seen by the camera
	pub fn draw_circle(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, radius: f64) {
		let position = self.transform * position;
		let radius = self.transform.scaling() * radius;
		DrawRenderer::aa_circle(canvas, position.x as i16, position.y as i16, radius as i16, color).unwrap()
	}
	/// Draws a filled circle as seen by the camera
	pub fn fill_circle(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, radius: f64) {
		let position = self.transform * position;
		let radius = self.transform.scaling() * radius;
		DrawRenderer::filled_circle(canvas, position.x as i16, position.y as i16, radius as i16, color).unwrap()
	}

	/// Draws ... as seen by the camera
	pub fn draw_stuff(&self, canvas: &mut Canvas<Window>, color: Color, n: usize) {}

	/// Draws the contour of a polygon from its vertices as seen by the camera
	pub fn draw_polygon(&self, canvas: &mut Canvas<Window>, color: Color, vertices: &Vec<Point2<f64>>) {
		let vertices: Vec<Point2<f64>> = vertices.iter().map(|point| self.transform * point).collect();
		let vx: Vec<i16> = vertices.iter().map(|point| point.x as i16).collect();
		let vy: Vec<i16> = vertices.iter().map(|point| point.y as i16).collect();
		DrawRenderer::aa_polygon(canvas, &vx, &vy, color).unwrap();
	}
	/// Draws a filled polygon from its vertices as seen by the camera
	pub fn fill_polygon(&self, canvas: &mut Canvas<Window>, color: Color, vertices: &Vec<Point2<f64>>) {
		let vertices: Vec<Point2<f64>> = vertices.iter().map(|point| self.transform * point).collect();
		let vx: Vec<i16> = vertices.iter().map(|point| point.x as i16).collect();
		let vy: Vec<i16> = vertices.iter().map(|point| point.y as i16).collect();
		DrawRenderer::filled_polygon(canvas, &vx, &vy, color).unwrap();
	}

	/// Draws a grid
	pub fn draw_grid(
		&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, color: Color, axes: bool, graduations: bool,
	) {
		let max_depth = 2;

		let p = (self.transform.scaling().log(5.0) + 1.4).floor();
		let global_scale = 5_f64.powf(p) / 100.0;
		let global_unit = |depth: i16| 5_f64.powf(depth as f64 - p) * 100.0;

		// Alignment
		let origin = self.transform * Point2::origin();
		let v_align = if origin.y.is_sign_negative() {
			VAlign::Top
		} else if (origin.y as u32).lt(&self.resolution.y) {
			VAlign::Center
		} else {
			VAlign::Bottom
		};
		let h_align = if origin.x.is_sign_negative() {
			HAlign::Left
		} else if (origin.x as u32).lt(&self.resolution.x) {
			HAlign::Center
		} else {
			HAlign::Right
		};
		let alignment = Align::from_align(
			if h_align == HAlign::Center { HAlign::Left } else { h_align },
			if v_align == VAlign::Center { VAlign::Top } else { v_align },
		);

		let x_transform = |x_th: i32, scale: f64| {
			(self.transform.scaling() / scale * x_th as f64 + self.transform.isometry.translation.x) as i16
		};
		let y_transform = |y_th: i32, scale: f64| {
			(self.transform.scaling() / scale * y_th as f64 + self.transform.isometry.translation.y) as i16
		};

		// Grid
		(0..=max_depth).for_each(|depth| {
			let line_color = darker(
				color,
				match depth {
					0 => 0.95,
					1 => 0.86,
					_ => 0.77,
				},
			);
			let scale = global_scale * 5_f64.powf(-depth as f64);
			let transform = self.transform.inverse().append_scaling(scale);

			let start = (transform * Point2::origin()).map(|v| v.ceil() as i32); // Top Left
			let end = (transform * Point2::from(self.resolution.cast())).map(|v| v.ceil() as i32); // Bottom Right

			(start.x..end.x).for_each(|x_th| {
				if (x_th % 5 != 0) | (depth == max_depth) {
					DrawRenderer::vline(canvas, x_transform(x_th, scale), 0, self.resolution.y as i16 - 1, line_color)
						.unwrap();
				}
			});
			(start.y..end.y).for_each(|y_th| {
				if (y_th % 5 != 0) | (depth == max_depth) {
					DrawRenderer::hline(canvas, 0, self.resolution.x as i16 - 1, y_transform(y_th, scale), line_color)
						.unwrap();
				}
			});
		});

		let axes_color = darker(color, 0.3);

		if axes {
			let x = match h_align {
				HAlign::Left => 0,
				HAlign::Center => origin.x as u32,
				HAlign::Right => self.resolution.x - 1,
			};
			let y = match v_align {
				VAlign::Top => 0,
				VAlign::Center => origin.y as u32,
				VAlign::Bottom => self.resolution.y - 1,
			};
			DrawRenderer::vline(canvas, x as i16, 0, self.resolution.y as i16 - 1, axes_color).unwrap();
			DrawRenderer::hline(canvas, 0, self.resolution.x as i16 - 1, y as i16, axes_color).unwrap();
		}

		if graduations {
			(1..=max_depth).for_each(|depth| {
				let scale = global_scale * 5_f64.powf(-depth as f64);
				let unit = global_unit(depth);

				let transform = self.transform.inverse().append_scaling(scale);

				let start = (transform * Point2::origin()).map(|v| v.ceil() as i32); // Top Left
				let end = (transform * Point2::from(self.resolution.cast())).map(|v| v.ceil() as i32); // Bottom Right

				let n = 8 * depth;
				let (x1, x2) = match h_align {
					HAlign::Left => (-n, n),
					HAlign::Center => (origin.x as i16 - n, origin.x as i16 + n),
					HAlign::Right => (self.resolution.x as i16 - 1 - n, self.resolution.x as i16 - 1 + n),
				};
				let (y1, y2) = match v_align {
					VAlign::Top => (-n, n),
					VAlign::Center => (origin.y as i16 - n, origin.y as i16 + n),
					VAlign::Bottom => (self.resolution.y as i16 - 1 - n, self.resolution.y as i16 - 1 + n),
				};

				let font_size = 16;
				let font_style = if depth == 1 { FontStyle::NORMAL } else { FontStyle::BOLD };
				let text_style = TextStyle::new(font_size, None, axes_color, font_style);

				(start.x..end.x).for_each(|x_th| {
					if (x_th % 5 != 0) | (depth == max_depth) {
						let x = x_transform(x_th, scale);
						DrawRenderer::vline(canvas, x, y1, y2, axes_color).unwrap();

						let position = Point::new(x as i32, (y1 as i32 + y2 as i32) / 2);
						let text = format!("{}", x_th as f64 * unit);
						text_drawer.draw(canvas, position, &text_style, &text, alignment);
					}
				});
				(start.y..end.y).for_each(|y_th| {
					if (y_th % 5 != 0) | (depth == max_depth) {
						let y = y_transform(y_th, scale);
						DrawRenderer::hline(canvas, x1, x2, y, axes_color).unwrap();

						let position = Point::new((x1 as i32 + x2 as i32) / 2, y as i32);
						let text = format!("{}", y_th as f64 * unit);
						text_drawer.draw(canvas, position, &text_style, &text, alignment);
					}
				});
			});
		}
	}
}
