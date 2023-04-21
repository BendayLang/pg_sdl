use crate::{Colors, draw_circle, point, rect};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub fn fill_background(canvas: &mut Canvas<Window>, color: Color) {
	canvas.set_draw_color(color);
	canvas.clear();
}


pub fn fill_rect(canvas: &mut Canvas<Window>, rect: Rect, radius: Option<u32>) {
	if let Some(radius) = radius {
		if 2 * radius >= rect.width() && 2 * radius >= rect.height() {
			canvas.set_draw_color(Colors::RED);
			canvas.fill_rect(rect).unwrap();
		} else if 2 * radius >= rect.width() {
			canvas.fill_rect(rect!(
			rect.x, rect.y + rect.w / 2,
			rect.w, rect.h - rect.w)).unwrap();
			
			let radius = rect.w as u32 / 2;
			let top = rect.top_left() + point!(radius, radius);
			let bottom = rect.bottom_left() + point!(radius, -(radius as i32 + 1));
			
			let (mut x, mut y, mut d) = (0, radius as i32, radius as i32 - 1);
			
			while y >= x {
				for (start_point, end_point) in [
					(top + point!(-x, -y), top + point!(x - 1, -y)),
					(top + point!(-y, -x), top + point!(y - 1, -x)),
					(bottom + point!(-x, y), bottom + point!(x - 1, y)),
					(bottom + point!(-y, x), bottom + point!(y - 1, x)),
				] { canvas.draw_line(start_point, end_point).unwrap(); }
				
				if d >= 2 * x {
					d -= 2 * x + 1;
					x += 1;
				} else if d < 2 * (radius as i32 - y) {
					d += 2 * y - 1;
					y -= 1;
				} else {
					d += 2 * (y - x - 1);
					y -= 1;
					x += 1;
				}
			}
		} else if 2 * radius >= rect.height() {
			canvas.fill_rect(rect!(
			rect.x + rect.h / 2, rect.y,
			rect.w - rect.h, rect.h)).unwrap();
			
			let radius = rect.h as u32 / 2 - 1;
			let left = rect.top_left() + point!(radius, radius);
			let right = rect.top_right() + point!(-(radius as i32 + 1), radius);
			
			let (mut x, mut y, mut d) = (0, radius as i32, radius as i32 - 1);
			
			while y >= x {
				for (start_point, end_point) in [
					(left + point!(-x, y + 1), left + point!(-x, -y)),
					(left + point!(-y, x + 1), left + point!(-y, -x)),
					(right + point!(x, y + 1), right + point!(x, -y)),
					(right + point!(y, x + 1), right + point!(y, -x)),
				] { canvas.draw_line(start_point, end_point).unwrap(); }
				
				if d >= 2 * x {
					d -= 2 * x + 1;
					x += 1;
				} else if d < 2 * (radius as i32 - y) {
					d += 2 * y - 1;
					y -= 1;
				} else {
					d += 2 * (y - x - 1);
					y -= 1;
					x += 1;
				}
			}
		} else {
			canvas.fill_rect(rect!(
			rect.x, rect.y + radius as i32 + 1,
			rect.w, rect.h - 2 * radius as i32 - 2)).unwrap();
			
			canvas.fill_rect(rect!(
			rect.x + radius as i32 + 1, rect.y,
			rect.w - 2 * radius as i32 - 2, radius as i32 + 1)).unwrap();
			canvas.fill_rect(rect!(
			rect.x + radius as i32 + 1, rect.y + rect.h - radius as i32 - 1,
			rect.w - 2 * radius as i32 - 2, radius as i32 + 1)).unwrap();
			
			let top_left = rect.top_left() + point!(radius, radius);
			let top_right = rect.top_right() + point!(-(radius as i32 + 1), radius);
			let bottom_left = rect.bottom_left() + point!(radius, -(radius as i32 + 1));
			let bottom_right = rect.bottom_right() + point!(-(radius as i32 + 1), -(radius as i32 + 1));
			
			let (mut x, mut y, mut d) = (0, radius as i32, radius as i32 - 1);
			
			while y >= x {
				for (start_point, end_point) in [
					(top_left + point!(-x, -y), top_left + point!(0, -y)),
					(top_left + point!(-y, -x), top_left + point!(0, -x)),
					(top_right + point!(0, -y), top_right + point!(x, -y)),
					(top_right + point!(0, -x), top_right + point!(y, -x)),
					(bottom_left + point!(-x, y), bottom_left + point!(0, y)),
					(bottom_left + point!(-y, x), bottom_left + point!(0, x)),
					(bottom_right + point!(0, y), bottom_right + point!(x, y)),
					(bottom_right + point!(0, x), bottom_right + point!(y, x)),
				] { canvas.draw_line(start_point, end_point).unwrap(); }
				
				if d >= 2 * x {
					d -= 2 * x + 1;
					x += 1;
				} else if d < 2 * (radius as i32 - y) {
					d += 2 * y - 1;
					y -= 1;
				} else {
					d += 2 * (y - x - 1);
					y -= 1;
					x += 1;
				}
			}
		}
	} else {
		canvas.fill_rect(rect).unwrap()
	}
}
