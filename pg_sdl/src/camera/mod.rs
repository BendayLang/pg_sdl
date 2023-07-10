use crate::canvas::{draw_rect, draw_rounded_rect, fill_rect, fill_rounded_rect};
use crate::color::{darker, Colors};
use crate::style::{Align, HAlign, VAlign};
use crate::text::{TextDrawer, TextStyle};
use crate::vector2::Vector2Plus;
use crate::{input::Input, point};
use nalgebra::{Matrix2, Matrix3, Point2, Scale2, Similarity2, Transform2, Translation2, Vector2};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::ttf::FontStyle;
use sdl2::video::Window;

pub struct Camera {
	pub transform: Similarity2<f64>,
	resolution: Vector2<u32>,
	scaling_factor: f64,
	min_scale: f64,
	max_scale: f64,
	top_limit: f64,
	bottom_limit: f64,
	left_limit: f64,
	right_limit: f64,
}

impl Camera {
	pub fn new(
		resolution: Vector2<u32>, doubling_steps: u8, zoom_in_limit: f64, zoom_out_limit: f64, top_limit: f64,
		bottom_limit: f64, left_limit: f64, right_limit: f64,
	) -> Self {
		Camera {
			resolution,
			transform: Similarity2::new(resolution.cast() * 0.5, 0.0, 1.0),
			scaling_factor: f64::powf(2.0, 1.0 / doubling_steps as f64),
			min_scale: 1.0 / zoom_out_limit,
			max_scale: zoom_in_limit,
			top_limit,
			bottom_limit,
			left_limit,
			right_limit,
		}
	}

	fn scale(&self) -> f64 {
		self.transform.scaling()
	}

	fn translation(&self) -> Vector2<f64> {
		self.transform.isometry.translation.vector
	}

	fn is_in_scope(&self, rect: Rect) -> bool {
		let camera_scope = Rect::new(0, 0, self.resolution.x, self.resolution.y);
		camera_scope.has_intersection(rect)
	}

	/// Translates and scales the camera from the inputs
	pub fn update(&mut self, input: &Input, lock_translation: bool) -> bool {
		let mut changed = false;

		if input.mouse.left_button.is_down() && !lock_translation {
			let mouse_delta = input.mouse.delta.cast();
			changed |= self.translate(mouse_delta);
		}

		let scaling = self.scaling_factor.powf(input.mouse.wheel as f64);
		let center = input.mouse.position.coords.cast();
		changed |= self.change_scale(scaling, center);

		changed
	}

	/// Translates the camera by 'delta' while restricting it within it limits.
	fn translate(&mut self, delta: Vector2<f64>) -> bool {
		if delta.is_empty() {
			return false;
		}
		let old_translation = self.translation();
		self.transform.append_translation_mut(&Translation2::from(delta));

		let start = self.transform.inverse() * Point2::origin(); // Top Left
		let end = self.transform.inverse() * Point2::from(self.resolution.cast()); // Bottom Right

		if start.x < self.left_limit {
			self.transform.isometry.translation.x = -self.left_limit * self.scale();
		}
		if start.y < self.top_limit {
			self.transform.isometry.translation.y = -self.top_limit * self.scale();
		}
		if end.x > self.right_limit {
			self.transform.isometry.translation.x = -self.right_limit * self.scale() + self.resolution.x as f64;
		}
		if end.y > self.bottom_limit {
			self.transform.isometry.translation.y = -self.bottom_limit * self.scale() + self.resolution.y as f64;
		}

		self.translation() != old_translation
	}

	/// Scales the camera by 'scaling' while restricting it within it limits.
	fn change_scale(&mut self, scaling: f64, center: Vector2<f64>) -> bool {
		if scaling == 1.0 {
			return false;
		}
		if self.min_scale > self.scale() * scaling {
			if self.scale() <= self.min_scale {
				return false;
			}
			let adjusted_scaling = self.min_scale / self.scale();
			self.transform.append_scaling_mut(adjusted_scaling);
			self.translate((1.0 - adjusted_scaling) * center);
			true
		} else if self.max_scale < self.scale() * scaling {
			if self.scale() >= self.max_scale {
				return false;
			}
			let adjusted_scaling = self.max_scale / self.scale();
			self.transform.append_scaling_mut(adjusted_scaling);
			self.translate((1.0 - adjusted_scaling) * center);
			true
		} else {
			self.transform.append_scaling_mut(scaling);
			self.translate((1.0 - scaling) * center);
			true
		}
	}

	fn resize(&mut self, new_resolution: Vector2<u32>) {
		// self.move((self.resolution - new_size) / self.scale / 2);
		self.translate((self.resolution - new_resolution).cast() / 2.0);
		self.resolution = new_resolution;
	}

	/// Draws a line as seen by the camera
	pub fn draw_line(&self, canvas: &mut Canvas<Window>, color: Color, start: Point2<f64>, end: Point2<f64>) {
		let start = self.transform * start;
		let end = self.transform * end;
		DrawRenderer::line(canvas, start.x as i16, start.y as i16, end.x as i16, end.y as i16, color).unwrap();
	}
	/// Draws a vertical line running the height of the screen and the x coordinate as seen by the camera
	pub fn draw_vline(&self, canvas: &mut Canvas<Window>, color: Color, x: f64) {
		let x = self.scale() * x + self.translation().x;
		DrawRenderer::vline(canvas, x as i16, 0, self.resolution.y as i16 - 1, color).unwrap();
	}
	/// Draws a horizontal line running the width of the screen and the y coordinate as seen by the camera
	pub fn draw_hline(&self, canvas: &mut Canvas<Window>, color: Color, y: f64) {
		let y = self.scale() * y + self.translation().y;
		DrawRenderer::hline(canvas, 0, self.resolution.x as i16 - 1, y as i16, color).unwrap();
	}

	/// Draws the contour of a rectangle as seen by the camera
	pub fn draw_rect(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>) {
		let position = self.transform * position;
		let size = self.transform * size;
		let rect = Rect::new(position.x as i32, position.y as i32, size.x as u32, size.y as u32);
		if self.is_in_scope(rect) {
			draw_rect(canvas, rect, color);
		};
	}
	/// Draws a filled rectangle as seen by the camera
	pub fn fill_rect(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>) {
		let position = self.transform * position;
		let size = self.transform * size;
		let rect = Rect::new(position.x as i32, position.y as i32, size.x as u32, size.y as u32);
		if self.is_in_scope(rect) {
			fill_rect(canvas, rect, color);
		};
	}

	/// Draws the contour of a rectangle as seen by the camera
	pub fn draw_rounded_rect(
		&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>, radius: f64,
	) {
		let position = self.transform * position;
		let size = self.transform * size;
		let rect = Rect::new(position.x as i32, position.y as i32, size.x as u32, size.y as u32);
		let radius = (self.transform.scaling() * radius) as u16;
		if self.is_in_scope(rect) {
			draw_rounded_rect(canvas, rect, color, radius);
		};
	}
	/// Draws a filled rectangle as seen by the camera
	pub fn fill_rounded_rect(
		&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, size: Vector2<f64>, radius: f64,
	) {
		let position = self.transform * position;
		let size = self.transform * size;
		let rect = Rect::new(position.x as i32, position.y as i32, size.x as u32, size.y as u32);
		let radius = (self.transform.scaling() * radius) as u16;
		if self.is_in_scope(rect) {
			fill_rounded_rect(canvas, rect, color, radius);
		};
	}

	/// Draws the contour of an ellipse as seen by the camera
	pub fn draw_ellipse(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, radii: Vector2<f64>) {
		let position = self.transform * position;
		let radii = self.transform * radii;
		let rect = Rect::new(
			(position.x - radii.x) as i32,
			(position.y - radii.y) as i32,
			2 * radii.x as u32,
			2 * radii.y as u32,
		);
		if self.is_in_scope(rect) {
			DrawRenderer::ellipse(canvas, position.x as i16, position.y as i16, radii.x as i16, radii.y as i16, color)
				.unwrap();
		};
	}
	/// Draws a filled ellipse as seen by the camera
	pub fn fill_ellipse(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, radii: Vector2<f64>) {
		let position = self.transform * position;
		let radii = self.transform * radii;
		let rect = Rect::new(
			(position.x - radii.x) as i32,
			(position.y - radii.y) as i32,
			2 * radii.x as u32,
			2 * radii.y as u32,
		);
		if self.is_in_scope(rect) {
			DrawRenderer::filled_ellipse(
				canvas,
				position.x as i16,
				position.y as i16,
				radii.x as i16,
				radii.y as i16,
				color,
			)
			.unwrap();
		};
	}

	/// Draws the contour of a circle as seen by the camera
	pub fn draw_circle(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, radius: f64) {
		let position = self.transform * position;
		let radius = self.scale() * radius;
		let rect =
			Rect::new((position.x - radius) as i32, (position.y - radius) as i32, 2 * radius as u32, 2 * radius as u32);
		if self.is_in_scope(rect) {
			DrawRenderer::circle(canvas, position.x as i16, position.y as i16, radius as i16, color).unwrap()
		};
	}
	/// Draws a filled circle as seen by the camera
	pub fn fill_circle(&self, canvas: &mut Canvas<Window>, color: Color, position: Point2<f64>, radius: f64) {
		let position = self.transform * position;
		let radius = self.scale() * radius;
		let rect =
			Rect::new((position.x - radius) as i32, (position.y - radius) as i32, 2 * radius as u32, 2 * radius as u32);
		if self.is_in_scope(rect) {
			DrawRenderer::filled_circle(canvas, position.x as i16, position.y as i16, radius as i16, color).unwrap()
		};
	}

	/// Draws ... as seen by the camera
	pub fn draw_stuff(&self, canvas: &mut Canvas<Window>, color: Color) {}

	/// Draws the contour of a polygon from its vertices as seen by the camera
	pub fn draw_polygon(&self, canvas: &mut Canvas<Window>, color: Color, vertices: &Vec<Point2<f64>>) {
		let vertices: Vec<Point2<f64>> = vertices.iter().map(|point| self.transform * point).collect();
		let vx: Vec<i16> = vertices.iter().map(|point| point.x as i16).collect();
		let vy: Vec<i16> = vertices.iter().map(|point| point.y as i16).collect();
		let x_min = *vx.iter().min().unwrap() as i32;
		let y_min = *vy.iter().min().unwrap() as i32;
		let x_max = *vx.iter().max().unwrap() as i32;
		let y_max = *vy.iter().max().unwrap() as i32;
		let rect = Rect::new(x_min, y_min, (x_max - x_min) as u32, (y_max - y_min) as u32);
		if self.is_in_scope(rect) {
			DrawRenderer::polygon(canvas, &vx, &vy, color).unwrap();
		}
	}
	/// Draws a filled polygon from its vertices as seen by the camera
	pub fn fill_polygon(&self, canvas: &mut Canvas<Window>, color: Color, vertices: &Vec<Point2<f64>>) {
		let vertices: Vec<Point2<f64>> = vertices.iter().map(|point| self.transform * point).collect();
		let vx: Vec<i16> = vertices.iter().map(|point| point.x as i16).collect();
		let vy: Vec<i16> = vertices.iter().map(|point| point.y as i16).collect();
		let x_min = *vx.iter().min().unwrap() as i32;
		let y_min = *vy.iter().min().unwrap() as i32;
		let x_max = *vx.iter().max().unwrap() as i32;
		let y_max = *vy.iter().max().unwrap() as i32;
		let rect = Rect::new(x_min, y_min, (x_max - x_min) as u32, (y_max - y_min) as u32);
		if self.is_in_scope(rect) {
			DrawRenderer::filled_polygon(canvas, &vx, &vy, color).unwrap();
		}
	}

	/// Draws an arrow as seen by the camera
	pub fn draw_arrow(
		&self, canvas: &mut Canvas<Window>, color: Color, start: Point2<f64>, end: Point2<f64>, width: f64,
	) {
		if start == end {
			return;
		}
		let start = self.transform * start;
		let end = self.transform * end;
		let width = self.transform.scaling() * width;
		// TODO clean up
		let x_dir = end - start;
		let y_dir = x_dir.perpendicular() * width / 2.0;
		let transform = Transform2::from_matrix_unchecked(Matrix3::new(
			x_dir.x, y_dir.x, start.x, x_dir.y, y_dir.y, start.y, 0.0, 0.0, 1.0,
		));

		let head_back: f64 = 1.0 - 3.0 * width / x_dir.norm();

		let mut points = Vec::from([
			Point2::new(head_back, -1.0),
			Point2::new(head_back, -3.0),
			Point2::new(1.0, 0.0),
			Point2::new(head_back, 3.0),
			Point2::new(head_back, 1.0),
		]);
		if x_dir.norm() > 3.0 * width {
			points.append(&mut Vec::from([Point2::new(0.0, 1.0), Point2::new(0.0, -1.0)]));
		}
		points.iter_mut().for_each(|v| *v = transform * *v);
		let points_x: Vec<i16> = points.iter().map(|v| v.x as i16).collect();
		let points_y: Vec<i16> = points.iter().map(|v| v.y as i16).collect();

		DrawRenderer::filled_polygon(canvas, &points_x, &points_y, color).unwrap();
		DrawRenderer::polygon(canvas, &points_x, &points_y, Colors::BLACK).unwrap();
	}

	/// Draws a grid
	pub fn draw_grid(
		&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, color: Color, axes: bool, graduations: bool,
	) {
		let max_depth = 2;

		let p = (self.scale().log(5.0) + 1.4).floor();
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

		let x_transform = |x_th: i32, scale: f64| (self.scale() / scale * x_th as f64 + self.translation().x) as i16;
		let y_transform = |y_th: i32, scale: f64| (self.scale() / scale * y_th as f64 + self.translation().y) as i16;

		// Grid
		(0..=max_depth).for_each(|depth| {
			let line_color = darker(
				color,
				match depth {
					0 => 0.96,
					1 => 0.88,
					_ => 0.80,
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

	/// Draws text as seen by the camera
	pub fn draw_text(
		&self, canvas: &mut Canvas<Window>, text_drawer: &TextDrawer, position: Point2<f64>, font_size: f64,
		text: String, align: Align,
	) {
		let position = self.transform * position;
		let position = Point::new(position.x as i32, position.y as i32);
		let text_style = &TextStyle { font_size: (self.scale() * font_size) as u16, ..Default::default() };
		text_drawer.draw(canvas, position, text_style, &text, align);
	}
}
