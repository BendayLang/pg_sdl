use sdl2::pixels::Color;

pub struct Text{
    pub text: String,
    pub size: f32,
    pub color: Color,
    pub font_index: u8,
}

impl Default for Text {
    fn default() -> Self {
        Self{
            text: "".to_string(),
            size: 20.,
            color: Color::BLACK,
            font_index: 0,
        }
    }
}
