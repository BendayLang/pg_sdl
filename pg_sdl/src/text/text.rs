use sdl2::pixels::Color;

pub struct Text{
    pub text: String,
    pub color: Color,
    pub font_size: f32,
    pub font_index: usize,
}

impl Text{
    pub fn new(text: String, font_size: f32) -> Self{
        Self{
            text,
            font_size,
            color: Color::BLACK,
            font_index: 0,
        }
    }
}
