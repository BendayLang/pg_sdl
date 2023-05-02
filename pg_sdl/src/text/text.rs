use sdl2::pixels::Color;

pub struct Text {
    pub text: String,
    pub color: Color,
    pub font_size: u16,
    pub font_path: String,
    pub font_style: sdl2::ttf::FontStyle,
}

impl Text {
    pub fn new(text: String, font_size: u16) -> Self {
        Self {
            text,
            font_size,
            font_path: "C:\\Windows\\Fonts\\Arial.ttf".to_string(),
            color: Color::BLACK,
            font_style: sdl2::ttf::FontStyle::NORMAL,
        }
    }
}
