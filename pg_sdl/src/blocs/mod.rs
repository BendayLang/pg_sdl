use crate::canvas::fill_rect;
use crate::text::TextDrawer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Bloc {
    color: Color,
    rect: Rect,
    corner_radius: Option<u32>,
    id: u32,
    parent: Option<u32>,
    child: Option<u32>,
}

static mut ID_COUNT: u32 = 0;

impl Bloc {
    pub fn new(color: Color, rect: Rect) -> Self {
        unsafe { ID_COUNT += 1 }
        Self {
            color,
            rect,
            corner_radius: None,
            id: unsafe { ID_COUNT.clone() },
            parent: None,
            child: None,
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        canvas.set_draw_color(self.color);
        fill_rect(canvas, self.rect, self.corner_radius);
    }
}
