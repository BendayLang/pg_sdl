use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::point;

pub struct Text{
    pub text: String,
    pub color: Color,
    pub font_size: f32,
    pub font_index: usize,
}

impl Default for Text {
    fn default() -> Self {
        Self{
            text: "text".to_string(),
            color: Color::BLACK,
            font_size: 20.,
            font_index: 0,
        }
    }
}
