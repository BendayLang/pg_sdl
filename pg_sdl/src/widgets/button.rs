use crate::prelude::*;
use crate::{
    canvas::draw_rounded_rect,
    canvas::fill_rect,
    color::{darker, Colors},
    input::{Input, KeyState},
    text::TextDrawer,
    widgets::Widget,
    widgets::{HOVER, PUSH},
};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// A button is a widget that it can be clicked.
pub struct Button {
    color: Color,
    hovered_color: Color,
    pushed_color: Color,
    rect: Rect,
    corner_radius: Option<u32>,
    text: Option<Text>,
    hovered: bool,
    pub state: KeyState,
}

impl Button {
    pub fn new(color: Color, rect: Rect, corner_radius: Option<u32>, text: Option<Text>) -> Self {
        Self {
            color,
            hovered_color: darker(color, HOVER),
            pushed_color: darker(color, PUSH),
            rect,
            corner_radius,
            text,
            hovered: false,
            state: KeyState::new(),
        }
    }
}

impl Widget for Button {
    fn update(&mut self, input: &Input, _delta: f32) -> bool {
        let mut changed = false;
        self.state.update();

        let hovered = self.rect.contains_point(input.mouse.position);
        if hovered != self.hovered {
            self.hovered = hovered;
            changed = true;
        }

        if input.mouse.left_button.is_pressed() && self.hovered {
            self.state.press();
            changed = true;
        } else if self.state.is_down() && input.mouse.left_button.is_released() {
            self.state.release();
            changed = true;
        }

        changed
    }

    fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        let color = if self.state.is_pressed() | self.state.is_down() {
            self.pushed_color
        } else if self.hovered {
            self.hovered_color
        } else {
            self.color
        };

        if let Some(radius) = self.corner_radius {
            DrawRenderer::rounded_box(
                canvas,
                self.rect.left() as i16,
                self.rect.top() as i16,
                self.rect.right() as i16,
                self.rect.bottom() as i16,
                radius as i16,
                color,
            )
            .expect("DrawRenderer failed");
            DrawRenderer::rounded_rectangle(
                canvas,
                self.rect.left() as i16,
                self.rect.top() as i16,
                self.rect.right() as i16,
                self.rect.bottom() as i16,
                radius as i16,
                Colors::BLACK,
            )
            .expect("DrawRenderer failed");
        } else {
            canvas.set_draw_color(color);
            canvas.fill_rect(self.rect).unwrap();
            canvas.set_draw_color(Colors::BLACK);
            canvas.draw_rect(self.rect).unwrap();
        }

        if let Some(text) = &self.text {
            text_drawer.draw(canvas, self.rect.center(), &text);
        }
    }
}
