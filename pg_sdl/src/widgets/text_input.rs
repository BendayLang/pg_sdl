use crate::canvas::draw_rect;
use crate::input::{KeyState, KeysState, Shortcut};
use crate::prelude::*;
use crate::widgets::{HOVER, PUSH};
use sdl2::keyboard::Keycode;

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
    carrot_last_update: f32,
    carrot_position: usize,
    carrot_visible: bool,
    selection: Option<(usize, usize)>,
    is_selecting: bool,
    pub state: KeyState,
}

impl TextInput {
    pub fn new(rect: Rect, style: Option<TextInputStyle>, default_text: Option<String>) -> Self {
        let carrot_position = match default_text {
            Some(ref text) => text.len(),
            None => 0,
        };
        Self {
            rect,
            style: style.unwrap_or_default(),
            content: default_text.unwrap_or_default(),
            hovered: false,
            state: KeyState::new(),
            is_focused: false,
            carrot_last_update: 0.0,
            carrot_position,
            carrot_visible: true,
            selection: None,
            is_selecting: false,
        }
    }
}

impl Widget for TextInput {
    fn update(&mut self, input: &Input, _delta: f32, text_drawer: &mut TextDrawer) -> bool {
        let mut changed = false;
        self.state.update();

        // Carrot blinking
        self.carrot_last_update += _delta;
        if self.carrot_last_update > 0.5 {
            self.carrot_last_update = 0.0;
            self.carrot_visible = !self.carrot_visible;
            changed = true;
        }

        // Mouse hover
        let hovered = self.rect.contains_point(input.mouse.position);
        if hovered != self.hovered {
            self.hovered = hovered;
            changed = true;
        }
        if hovered {
            // Mouse click
            if input.mouse.left_button.is_pressed() {
                let mouse_x = input.mouse.position.x - self.rect.x;
                let mouse_y = input.mouse.position.y - self.rect.y;
                let mut min_distance = std::i32::MAX;
                let mut new_carrot_position = 0;
                for (i, c) in self.content.chars().enumerate() {
                    let text = &self.content[..i];
                    let (h, w) = text_drawer.text_size(&self.style.text_style, &text);
                    let distance = (w as i32 - mouse_x).abs();

                    if distance < min_distance {
                        min_distance = distance;
                        new_carrot_position = i;
                    }
                }
                self.carrot_position = new_carrot_position;

                self.state.press();
                self.is_focused = true;
                self.is_selecting = true;
                changed = true;
            }
        }

        if self.is_selecting && input.mouse.left_button.is_released() {
            self.is_selecting = false;
            changed = true;
        }

        if input.mouse.left_button.is_pressed() && !self.hovered {
            self.state.release();
            self.is_focused = false;
            changed = true;
        } else if self.state.is_down() && input.mouse.left_button.is_released() {
            self.state.release();
            changed = true;
        }

        // Keyboard input
        if self.is_focused {
            // Clipboard
            if input.shortcut_pressed(&Shortcut::PASTE()) && input.clipboard.has_clipboard_text() {
                let clipboard_text = input.clipboard.clipboard_text().unwrap();
                self.content
                    .insert_str(self.carrot_position, &clipboard_text);
                self.carrot_position = self.carrot_position + clipboard_text.len();
                return true;
            }
            if input.shortcut_pressed(&Shortcut::COPY()) {
                input.clipboard.set_clipboard_text(&self.content).unwrap();
                return true;
            }
            if input.shortcut_pressed(&Shortcut::CUT()) {
                input.clipboard.set_clipboard_text(&self.content).unwrap();
                self.content.clear();
                self.carrot_position = 0;
                return true;
            }

            // Text input
            if let Some(c) = input.last_char {
                changed = true;
                self.content.insert(self.carrot_position, c);
                if self.carrot_position < self.content.len() {
                    self.carrot_position += 1;
                }
            }
            if input.keys_state.backspace.is_pressed() {
                changed = true;
                if self.carrot_position > 0 {
                    self.content.remove(self.carrot_position - 1);
                    self.carrot_position -= 1;
                }
            }

            // Carrot movement
            if input.keys_state.left.is_pressed() {
                changed = true;
                if self.carrot_position > 0 {
                    self.carrot_position -= 1;
                }
            }
            if input.keys_state.right.is_pressed() {
                changed = true;
                if self.carrot_position < self.content.len() {
                    self.carrot_position += 1;
                }
            }
        }

        changed
    }

    fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        // Box
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

        // Text
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

        // Carrot
        if self.is_focused && self.carrot_visible {
            let carrot_x_position = if self.carrot_position != 0 {
                text_drawer
                    .text_size(
                        &self.style.text_style,
                        &self.content[..self.carrot_position],
                    )
                    .1 as i32
            } else {
                0
            };

            let carrot_rect = Rect::new(
                self.rect.left() + 5 + carrot_x_position,
                self.rect.top() + 5,
                1,
                self.rect.height() - 10,
            );
            fill_rect(canvas, carrot_rect, Colors::BLACK, None);
        }
    }
}
