mod fonts_init;
use std::collections::HashMap;

use fontdue;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue_sdl2::FontTexture;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
pub struct TextDrawer {
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub fonts: Vec<fontdue::Font>,
    pub fonts_id: HashMap<String, usize>,
}

impl TextDrawer {
    pub fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        let (fonts, fonts_id) = fonts_init::fonts_init();
        println!("{:?}", fonts_id);
        TextDrawer {
            texture_creator,
            fonts,
            fonts_id,
        }
    }

    pub fn draw_text(
        &mut self,
        canvas: &mut Canvas<Window>,
        font_index: usize,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
    ) {
        let mut font_texture = FontTexture::new(&self.texture_creator).unwrap();
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y,
            ..Default::default()
        });
        layout.append(
            &self.fonts,
            &TextStyle::with_user_data(text, font_size, font_index, Color::RGB(0xFF, 0xFF, 0)),
        );
        font_texture
            .draw_text(canvas, &self.fonts, layout.glyphs())
            .unwrap();
    }
}
