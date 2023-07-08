use crate::prelude::*;

pub fn fill_background(canvas: &mut Canvas<Window>, color: Color) {
	canvas.set_draw_color(color);
	canvas.clear();
}

pub fn draw_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color) {
	canvas.set_draw_color(color);
	canvas.draw_rect(rect).unwrap();
}
pub fn fill_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color) {
	canvas.set_draw_color(color);
	canvas.fill_rect(rect).unwrap();
}

pub fn draw_rounded_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color, radius: u16) {
	let (x1, x2) = (rect.left(), rect.right() - 1);
	let (y1, y2) = (rect.top(), rect.bottom() - 1);
	DrawRenderer::rounded_rectangle(canvas, x1 as i16, y1 as i16, x2 as i16, y2 as i16, radius as i16, color).unwrap();
}
pub fn fill_rounded_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color, radius: u16) {
	let (x1, x2) = (rect.left(), rect.right() - 1);
	let (y1, y2) = (rect.top(), rect.bottom() - 1);
	DrawRenderer::rounded_box(canvas, x1 as i16, y1 as i16, x2 as i16, y2 as i16, radius as i16, color).unwrap();
}
