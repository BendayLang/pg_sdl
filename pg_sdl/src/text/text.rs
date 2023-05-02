use sdl2::pixels::Color;
use std::path::Path;

static FONT_PATH: &str = "fonts/";
static DEFAULT_FONT_NAME: &str = "Vera.ttf";

pub struct Text {
    pub text: String,
    pub color: Color,
    pub font_size: u16,
    pub font_name: String,
    pub font_style: sdl2::ttf::FontStyle,
}

impl Text {
    pub fn new(text: String, font_size: u16, font_name: Option<&str>) -> Self {
        let font_name = if let Some(font_name) = font_name {
            let font_name = format!("{}Vera.ttf", FONT_PATH);
            if !Path::new(&font_name).exists() {
                format!("{}DejaVuSans.ttf", FONT_PATH);
            }
            font_name
        } else {
            format!("{}{}", FONT_PATH, DEFAULT_FONT_NAME)
        };

        Self {
            text,
            font_size,
            font_name,
            color: Color::BLACK,
            font_style: sdl2::ttf::FontStyle::NORMAL,
        }
    }
}
