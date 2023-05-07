use crate::input::KeyState;
use crate::prelude::*;
use crate::widgets::{HOVER, PUSH};
use sdl2::gfx::primitives::DrawRenderer;

pub struct TextInput {
    color: Color,
    hovered_color: Color,
    pushed_color: Color,
    rect: Rect,
    corner_radius: Option<u32>,
    text: Option<Text>,
    hovered: bool,
    pub state: KeyState,
}

impl TextInput {
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

impl Widget for TextInput {
    fn update(&mut self, input: &Input, _delta: f32) -> bool {
        let mut changed = false;
        self.state.update();

        let hovered = self.rect.contains_point(input.mouse.position);
        if hovered != self.hovered {
            self.hovered = hovered;
            changed = true;
        }

        if self.hovered && input.mouse.left_button.is_pressed() {
            self.state.press();
            changed = true;
        } else if !self.hovered && input.mouse.left_button.is_pressed() {
            self.state.release();
            changed = true;
        }

        if let Some(c) = input.last_char {
            changed = true;
            if c == '\u{8}' {
                // backspace
                if let Some(text) = &mut self.text {
                    text.text.pop();
                }
            } else if c == '\u{1B}' {
                // escape
                if let Some(text) = &mut self.text {
                    text.text.clear();
                }
            } else if c == '\u{D}' {
                // enter
            } else {
                if let Some(text) = &mut self.text {
                    text.text.push(c);
                }
            }
        }

        changed
    }

    fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        let color = if self.state.is_pressed() {
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
        } else {
            fill_rect(canvas, self.rect, None);
        }

        if let Some(text) = &self.text {
            text_drawer.draw(canvas, point!(self.rect.left(), self.rect.top()), text);
        }
    }
}
