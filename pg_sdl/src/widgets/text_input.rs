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
    const LEFT_SHIFT: i32 = 5;

    fn get_carrot_position_from_mouse(
        &self,
        text_drawer: &mut TextDrawer,
        mouse_x: i32,
    ) -> Option<usize> {
        let mut x: u32 = 0;
        for (i, c) in self.content.chars().enumerate() {
            let text_width = text_drawer
                .text_size(&self.style.text_style, &c.to_string())
                .0;
            x += text_width;
            if x >= mouse_x as u32 {
                return Some(i);
            }
        }
        return None;
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
            if input.mouse.left_button_double_clicked() {
                self.selection = Some((0, self.content.len()));
                changed = true;
            }
            else if input.mouse.left_button.is_pressed() {
                self.selection = None;

                // Carrot position
                let mouse_x = input.mouse.position.x - self.rect.x;
                self.carrot_position = if let Some(new_carrot_position) =
                    self.get_carrot_position_from_mouse(text_drawer, mouse_x)
                {
                    new_carrot_position
                } else {
                    self.content.len()
                };

                // Selection
                self.state.press();
                self.is_focused = true;
                self.is_selecting = true;
                changed = true;
            } else if input.mouse.left_button.is_down() && self.is_selecting {
                // Selection
                let mouse_x = input.mouse.position.x - self.rect.x;
                let new_carrot_position = if let Some(new_carrot_position) =
                    self.get_carrot_position_from_mouse(text_drawer, mouse_x)
                {
                    new_carrot_position
                } else {
                    self.content.len()
                };
                if new_carrot_position != self.carrot_position {
                    if self.carrot_position > new_carrot_position {
                        self.selection = Some((new_carrot_position, self.carrot_position));
                    } else {
                        self.selection = Some((self.carrot_position, new_carrot_position));
                    }
                    changed = true;
                }
                //println!("Selection: {:?}", self.selection);
            }
        }

        if self.is_selecting && input.mouse.left_button.is_released() {
            self.is_selecting = false;
            changed = true;
        }

        if input.mouse.left_button.is_pressed() && !self.hovered {
            self.state.release();
            self.is_focused = false;
            self.selection = None;
            changed = true;
        } else if self.state.is_down() && input.mouse.left_button.is_released() {
            self.state.release();
            changed = true;
        }

        // Keyboard input
        if self.is_focused {
            // Clipboard
            if input.shortcut_pressed(&Shortcut::PASTE()) && input.clipboard.has_clipboard_text() {
                if self.selection.is_some() {
                    let (start, end) = self.selection.unwrap();
                    self.content.drain(start..end);
                    self.carrot_position = start;
                    self.selection = None;
                }
                let clipboard_text = input.clipboard.clipboard_text().unwrap();
                self.content
                    .insert_str(self.carrot_position, &clipboard_text);
                self.carrot_position = self.carrot_position + clipboard_text.len();
                return true;
            }
            if input.shortcut_pressed(&Shortcut::COPY()) {
                if self.selection.is_some() {
                    let (start, end) = self.selection.unwrap();
                    let text = self.content[start..end].to_string();
                    input.clipboard.set_clipboard_text(&text).unwrap();
                    return true;
                }
                input.clipboard.set_clipboard_text(&self.content).unwrap();
                return true;
            }
            if input.shortcut_pressed(&Shortcut::CUT()) {
                if self.selection.is_some() {
                    let (start, end) = self.selection.unwrap();
                    let text = self.content.drain(start..end).collect::<String>();
                    input.clipboard.set_clipboard_text(&text).unwrap();
                    self.carrot_position = start;
                    self.selection = None;
                    return true;
                }
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
                if self.selection.is_some() {
                    let (start, end) = self.selection.unwrap();
                    self.content.drain(start..end);
                    self.carrot_position = start;
                    self.selection = None;
                } else if self.carrot_position > 0 {
                    self.content.remove(self.carrot_position - 1);
                    self.carrot_position -= 1;
                }
                changed = true;
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
        let background_color = if self.hovered {
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
                    self.rect.left() + Self::LEFT_SHIFT,
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
                    .0 as i32
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

        // Selection
        if let Some(selection) = self.selection {
            let selection_rect = Rect::new(
                self.rect.left()
                    + 5
                    + text_drawer
                    .text_size(&self.style.text_style, &self.content[..selection.0])
                    .0 as i32,
                self.rect.top() + 5,
                text_drawer
                    .text_size(
                        &self.style.text_style,
                        &self.content[selection.0..selection.1],
                    )
                    .0 as u32,
                self.rect.height() - 10,
            );
            fill_rect(canvas, selection_rect, Colors::BLUE, None);
        }
    }
}
