use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct WindowCanvas(pub Canvas<Window>);

impl WindowCanvas {
    pub fn fill_background(&mut self, color: Color) {
        self.0.set_draw_color(color);
        self.0.clear();
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.0.set_draw_color(color);
        self.0.fill_rect(rect).unwrap();
    }
}
