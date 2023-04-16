use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::canvas::fill_rect;
use crate::{Input, rect};
use crate::input::KeyState;
use crate::text::TextDrawer;
use crate::text::Text;


trait Widget{

}

pub struct Button {
    pub color: Color,
    pub rect: Rect,
    pub corner_radius: Option<f32>,
    pub text: Option<Text>,
    pub state: KeyState,
    pub hovered: bool,
    pub changed: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self{
            color: Color::GREY,
            rect: rect!(0,0,1,1),
            corner_radius: Some(0.2),
            text: None,
            state: KeyState::new(),
            hovered: false,
            changed: false,
        }
    }
}

impl Button {
    pub fn new(color: Color, rect: Rect, text: Option<Text>) -> Self {
        Self {
            color,
            rect,
            text,
            ..Default::default()
        }
    }
    pub fn update(&mut self, input: &Input, delta: f32) {
        self.state.update();

        let new_hovered = self.rect.contains_point(input.mouse.position);
        if new_hovered != self.hovered {
            self.hovered = new_hovered;
            self.changed = true;
        }

        if input.mouse.left_button.is_pressed() && self.hovered {
            self.state.press();
            self.changed = true;
        } else if self.state.is_down() && input.mouse.left_button.is_released() {
            self.state.release();
            self.changed = true;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        let color = if self.state.is_down() {
            darker(self.color, 0.7)
        } else if self.hovered {
            darker(self.color, 0.9)
        } else {
            self.color
        };
        fill_rect(canvas, self.rect, color, None);
        if let Some(text) = &self.text{
            text_drawer.draw(canvas,
                             0,
                             &text.text,
                             self.rect.center(),
                             text.size,
                             text.color,
            );
        }

    }
}


fn darker(color: Color, value_change: f32) -> Color {
    Color::RGB((color.r as f32 * value_change) as u8,
               (color.g as f32 * value_change) as u8,
               (color.b as f32 * value_change) as u8)
}