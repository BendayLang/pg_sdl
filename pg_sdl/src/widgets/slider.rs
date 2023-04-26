use crate::input::KeyState;
use crate::prelude::*;
use crate::widgets::{HOVER, PUSH};
use fontdue::layout::{HorizontalAlign, VerticalAlign};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt::Display;

pub enum Orientation {
    Horizontal,
    Vertical,
}

/// A slider can be:
///
/// **discrete** with a number of **snap** points or **continuous**
///
/// It has:
/// - a **default value**
/// - a **value getter function** which is applied to the value of the slider when you call **get_value()**
pub enum SliderType<R: ?Sized + Display> {
    Discrete {
        snap: u32,
        default_value: u32,
        value_getter_function: Box<dyn Fn(u32) -> Box<R>>,
    },
    Continuous {
        default_value: f32,
        value_getter_function: Box<dyn Fn(f32) -> Box<R>>,
    },
}

/// A slider is a widget that can be dragged to change a value.
///
/// It can be discrete or continuous
pub struct Slider<R: ?Sized + Display> {
    color: Color,
    hovered_color: Color,
    back_color: Color,
    hovered_back_color: Color,
    thumb_color: Color,
    hovered_thumb_color: Color,
    pushed_thumb_color: Color,
    rect: Rect,
    orientation: Orientation,
    corner_radius: Option<u32>,
    hovered: bool,
    pub state: KeyState,
    /// private value of the slider (0.0 - 1.0)
    value: f32,
    slider_type: SliderType<R>,
    draw_value: bool,
}

impl<R: ?Sized + Display> Slider<R> {
    pub fn new(
        color: Color,
        rect: Rect,
        corner_radius: Option<u32>,
        slider_type: SliderType<R>,
        draw_value: bool,
    ) -> Self {
        let orientation = {
            if rect.width() > rect.height() {
                Orientation::Horizontal
            } else {
                Orientation::Vertical
            }
        };
        let thumb_color = Colors::LIGHT_GREY;
        let back_color = darker(paler(color, 0.5), 0.9);
        Self {
            color,
            hovered_color: darker(color, HOVER),
            back_color,
            hovered_back_color: darker(back_color, HOVER),
            thumb_color,
            hovered_thumb_color: darker(thumb_color, HOVER),
            pushed_thumb_color: darker(thumb_color, PUSH),
            rect,
            orientation,
            corner_radius,
            hovered: false,
            state: KeyState::new(),
            value: match slider_type {
                SliderType::Discrete {
                    default_value,
                    snap,
                    ..
                } => default_value as f32 / snap as f32,
                SliderType::Continuous { default_value, .. } => default_value,
            },
            slider_type,
            draw_value,
        }
    }

    pub fn get_value(&self) -> Box<R> {
        match &self.slider_type {
            SliderType::Discrete {
                snap,
                value_getter_function,
                ..
            } => {
                let value = (self.value * *snap as f32).round() as u32;
                value_getter_function(value)
            }
            SliderType::Continuous {
                value_getter_function,
                ..
            } => value_getter_function(self.value),
        }
    }

    pub fn reset_value(&mut self) {
        self.value = match &self.slider_type {
            SliderType::Discrete {
                snap,
                default_value,
                ..
            } => *default_value as f32 / *snap as f32,
            SliderType::Continuous { default_value, .. } => *default_value,
        };
    }

    fn thumb_position(&self) -> u32 {
        (self.value * self.length() as f32) as u32
    }

    fn length(&self) -> u32 {
        match self.orientation {
            Orientation::Horizontal => self.rect.width() - self.rect.height(),
            Orientation::Vertical => self.rect.height() - self.rect.width(),
        }
    }

    fn thickness(&self) -> u32 {
        match self.orientation {
            Orientation::Horizontal => self.rect.height(),
            Orientation::Vertical => self.rect.width(),
        }
    }
}

impl<R: ?Sized + Display> Widget for Slider<R> {
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

        if self.state.is_pressed() | self.state.is_down() {
            let value = {
                let point = input.mouse.position;
                let thumb_position = match self.orientation {
                    Orientation::Horizontal => point.x() - self.rect.left(),
                    Orientation::Vertical => self.rect.bottom() - point.y(),
                } - self.thickness() as i32 / 2;
                thumb_position.clamp(0, self.length() as i32) as f32 / self.length() as f32
            };

            let value = match self.slider_type {
                SliderType::Discrete { snap, .. } => (value * snap as f32).round() / snap as f32,
                SliderType::Continuous { .. } => value,
            };

            if value != self.value {
                self.value = value;
                changed = true;
            }
        }

        changed
    }

    fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        let b: f32 = 0.7;

        // Back bar
        let margin = (self.thickness() as f32 * (1.0 - b) / 2.0) as u32;

        let (back_rect, rect) = match self.orientation {
            Orientation::Horizontal => (
                rect!(
                    self.rect.left() + (self.thumb_position() + self.thickness() / 2) as i32,
                    self.rect.top() + margin as i32,
                    self.rect.width()
                        - self.thumb_position() as u32
                        - self.thickness() / 2
                        - margin,
                    self.rect.height() as f32 * b
                ),
                rect!(
                    self.rect.left() + margin as i32,
                    self.rect.top() + margin as i32,
                    self.thumb_position() + self.thickness() / 2 - margin,
                    self.rect.height() as f32 * b
                ),
            ),
            Orientation::Vertical => (
                rect!(
                    self.rect.left() + margin as i32,
                    self.rect.top() + margin as i32,
                    self.rect.width() as f32 * b,
                    self.rect.height() - self.thumb_position() - self.thickness() / 2 - margin
                ),
                rect!(
                    self.rect.left() + margin as i32,
                    self.rect.bottom() - (self.thumb_position() + self.thickness() / 2) as i32,
                    self.rect.width() as f32 * b,
                    self.thumb_position() + self.thickness() / 2 - margin
                ),
            ),
        };

        canvas.set_draw_color(
            if self.hovered | self.state.is_pressed() | self.state.is_down() {
                self.hovered_back_color
            } else {
                self.back_color
            },
        );
        fill_rect(canvas, back_rect, self.corner_radius);
        canvas.set_draw_color(
            if self.hovered | self.state.is_pressed() | self.state.is_down() {
                self.hovered_color
            } else {
                self.color
            },
        );
        fill_rect(canvas, rect, self.corner_radius);

        // Pad
        canvas.set_draw_color(if self.state.is_pressed() | self.state.is_down() {
            self.pushed_thumb_color
        } else if self.hovered {
            self.hovered_thumb_color
        } else {
            self.thumb_color
        });
        let rect = match self.orientation {
            Orientation::Horizontal => rect!(
                self.rect.left() + self.thumb_position() as i32,
                self.rect.top(),
                self.thickness(),
                self.thickness()
            ),
            Orientation::Vertical => rect!(
                self.rect.left(),
                self.rect.bottom() - self.thumb_position() as i32 - self.thickness() as i32,
                self.thickness(),
                self.thickness()
            ),
        };
        fill_rect(canvas, rect, self.corner_radius);

        if self.draw_value {
            let text = format!("{}", self.get_value());
            text_drawer.draw(
                canvas,
                &Text::new(text, 20.0),
                rect.center(),
                None,
                None,
                HorizontalAlign::Left,
                VerticalAlign::Top,
            );
        }
    }
}
