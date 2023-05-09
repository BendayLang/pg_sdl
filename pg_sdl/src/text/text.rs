use crate::prelude::*;
use std::path::Path;

static FONT_PATH: &str = "fonts/";
static DEFAULT_FONT_NAME: &str = "Vera.ttf";

pub struct TextStyle {
    // pub text: String,
    pub color: Color,
    pub font_size: u16,
    pub font_name: String,
    pub font_style: sdl2::ttf::FontStyle,
    pub h_align: Align,
}

impl TextStyle {
    pub fn new(font_size: u16, font_name: Option<&str>) -> Self {
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
            // text,
            font_size,
            font_name,
            color: Color::BLACK,
            font_style: sdl2::ttf::FontStyle::NORMAL,
            h_align: Align::Start,
        }
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            // text: String::new(),
            font_size: 16,
            font_name: format!("{}{}", FONT_PATH, DEFAULT_FONT_NAME),
            color: Color::BLACK,
            font_style: sdl2::ttf::FontStyle::NORMAL,
            h_align: Align::Start,
        }
    }
}
