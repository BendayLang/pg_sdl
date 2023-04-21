use crate::point;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub fn fill_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32) {
	let (mut x, mut y, mut d) = (0, radius, radius - 1);
	
	while y >= x {
		for (start_point, end_point) in [
			(point!(-x, -y), point!(x, -y)),
			(point!(-y, -x), point!(y, -x)),
			(point!(-x, y), point!(x, y)),
			(point!(-y, x), point!(y, x)),
		] { canvas.draw_line(center + start_point, center + end_point).unwrap(); }
		
		if d >= 2 * x {
			d -= 2 * x + 1;
			x += 1;
		} else if d < 2 * (radius - y) {
			d += 2 * y - 1;
			y -= 1;
		} else {
			d += 2 * (y - x - 1);
			y -= 1;
			x += 1;
		}
	}
}

pub fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32, width: i32) {
	let (mut x, mut y, mut d) = (0, radius, radius - 1);
	
	while y >= x {
		for point in [
			point!(-x, -y), point!(x, -y),
			point!(-y, -x), point!(y, -x),
			point!(-x, y), point!(x, y),
			point!(-y, x), point!(y, x),
		] { canvas.draw_point(center + point).unwrap(); }
		
		if d >= 2 * x {
			d -= 2 * x + 1;
			x += 1;
		} else if d < 2 * (radius - y) {
			d += 2 * y - 1;
			y -= 1;
		} else {
			d += 2 * (y - x - 1);
			y -= 1;
			x += 1;
		}
	}
}
