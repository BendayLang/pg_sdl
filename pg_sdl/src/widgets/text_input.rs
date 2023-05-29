use crate::canvas::draw_rect;
use crate::input::KeyState;
use crate::prelude::*;
use crate::widgets::{HOVER, PUSH};

pub struct TextInputStyle {
    background_color: Color,
    background_hovered_color: Color,
    background_pushed_color: Color,
    contour_color: Color,
    contour_focused_color: Color,
    corner_radius: Option<u16>,
    text_style: TextStyle,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            background_color: Colors::WHITE,
            background_hovered_color: darker(Colors::WHITE, HOVER),
            background_pushed_color: darker(Colors::WHITE, PUSH),
            contour_color: Colors::BLACK,
            contour_focused_color: paler(Colors::BLUE, 0.9),
            corner_radius: Some(4),
            text_style: TextStyle::default(),
        }
    }
}

pub struct TextInput {
    rect: Rect,
    style: TextInputStyle,
    content: String,
    hovered: bool,
    is_focused: bool,
    pub state: KeyState,
}

impl TextInput {
    pub fn new(rect: Rect, style: Option<TextInputStyle>, default_text: Option<String>) -> Self {
        Self {
            rect,
            style: style.unwrap_or_default(),
            content: default_text.unwrap_or_default(),
            hovered: false,
            state: KeyState::new(),
            is_focused: false,
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

        if input.mouse.left_button.is_pressed() && self.hovered {
            self.state.press();
            self.is_focused = true;
            changed = true;
        } else if input.mouse.left_button.is_pressed() && !self.hovered {
            self.state.release();
            self.is_focused = false;
            changed = true;
        } else if self.state.is_down() && input.mouse.left_button.is_released() {
            self.state.release();
            changed = true;
        }

        if !self.is_focused {
            return changed;
        }

        if let Some(c) = input.last_char {
            changed = true;
            self.content.push(c);
        }
        if input.keys_state.backspace.is_pressed() {
            changed = true;
            self.content.pop();
        }

        changed
    }

    fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        let background_color = if self.state.is_pressed() | self.state.is_down() {
            self.style.background_pushed_color
        } else if self.hovered {
            self.style.background_hovered_color
        } else {
            self.style.background_color
        };
        let contour_color = if self.is_focused {
            self.style.contour_focused_color
        } else {
            self.style.contour_color
        };
        fill_rect(
            canvas,
            self.rect,
            background_color,
            self.style.corner_radius,
        );
        draw_rect(
            canvas,
            self.rect,
            self.style.contour_color,
            self.style.corner_radius,
        );
        if self.is_focused {
            let rect = Rect::new(
                self.rect.left() + 1,
                self.rect.top() + 1,
                self.rect.width() - 2,
                self.rect.height() - 2,
            );
            let corner_radius = self.style.corner_radius.map(|r| r - 1);
            draw_rect(
                canvas,
                rect,
                self.style.contour_focused_color,
                corner_radius,
            );
        }

        if !self.content.is_empty() {
            text_drawer.draw(
                canvas,
                point!(
                    self.rect.left() + 5,
                    self.rect.height() as i32 / 2 + self.rect.top()
                ),
                &self.style.text_style,
                &self.content,
                Align::Left,
            );
        }
    }
}
