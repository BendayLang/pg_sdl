use crate::canvas::draw_rect;
use crate::canvas::fill_rect;
use crate::text::TextDrawer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

/// A bloc represents a piece of code that can be executed.
pub struct Bloc {
    color: Color,
    /// If the bloc has a parent, the rect position is relative to the parent.
    rect: Rect,
    corner_radius: Option<u32>,
    parent: Option<u32>,
    child: Option<u32>,
}

impl Bloc {
    pub fn new(color: Color, rect: Rect) -> Self {
        Self {
            color,
            rect,
            corner_radius: None,
            parent: None,
            child: None,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        canvas.set_draw_color(self.color);
        fill_rect(canvas, self.rect, self.corner_radius);
        canvas.set_draw_color(Color::BLACK);
        draw_rect(canvas, self.rect);
    }

    pub fn move_by(&mut self, point: Point) {
        self.rect.reposition(self.rect.top_left() + point);
    }

    pub fn collide(&self, point: Point) -> bool {
        self.rect.contains_point(point)
    }

    pub fn collide_bloc(&self, bloc: &Bloc) -> bool {
        self.rect.has_intersection(bloc.rect)
    }

    pub fn get_parent(&self) -> Option<u32> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: u32) {
        self.parent = Some(parent);
    }

    pub fn get_child(&self) -> Option<u32> {
        self.child
    }

    pub fn set_child(&mut self, child_id: u32) {
        self.child = Some(child_id);
        // self.rect.set_width(self.rect.width() + child.rect.width());
        // self.rect.set_height(self.rect.height() + child.rect.height());
    }
}
