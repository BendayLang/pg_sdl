use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub fn fill_background(canvas: &mut Canvas<Window>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
}

pub fn draw_rect(canvas: &mut Canvas<Window>, rect: Rect, color: Color) {
    canvas.set_draw_color(color);
    canvas.fill_rect(rect).unwrap();
}
