use crate::input::KeyState;
use crate::prelude::*;

pub struct TextInputStyle {
    pub background_color: Color,
    pub corner_radius: Option<u16>,
    pub text_style: TextStyle,
    pub rect: Rect,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            corner_radius: Some(5),
            text_style: TextStyle::default(),
            rect: Rect::new(100, 100, 120, 30),
        }
    }
}

pub struct TextInput {
    style: TextInputStyle,
    content: String,
    hovered: bool,
    is_focused: bool,
    pub state: KeyState,
}

impl TextInput {
    pub fn new(style: Option<TextInputStyle>, default_text: Option<String>) -> Self {
        Self {
            hovered: false,
            state: KeyState::new(),
            style: style.unwrap_or_default(),
            content: default_text.unwrap_or_default(),
            is_focused: false,
        }
    }
}

impl Widget for TextInput {
    fn update(&mut self, input: &Input, _delta: f32) -> bool {
        let mut changed = false;
        self.state.update();

        let hovered = self.style.rect.contains_point(input.mouse.position);
        if hovered != self.hovered {
            self.hovered = hovered;
            changed = true;
        }

        if self.hovered && input.mouse.left_button.is_pressed() {
            self.state.press();
            self.is_focused = true;
            changed = true;
        } else if !self.hovered && input.mouse.left_button.is_pressed() {
            self.state.release();
            self.is_focused = false;
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
        let color = if !self.is_focused {
            darker(self.style.background_color, 0.9)
        } else if self.state.is_pressed() {
            darker(self.style.background_color, 0.8)
        } else if self.hovered {
            darker(self.style.background_color, 0.7)
        } else {
            self.style.background_color
        };

        fill_rect(canvas, self.style.rect, color, self.style.corner_radius);

        if !self.content.is_empty() {
            text_drawer.draw(
                canvas,
                point!(
                    self.style.rect.left() + 5,
                    self.style.rect.height() as i32 / 2 + self.style.rect.top()
                ),
                &self.style.text_style,
                &self.content,
            );
        }
    }
}
