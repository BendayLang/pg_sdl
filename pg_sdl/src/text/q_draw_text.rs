use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue_sdl2::FontTexture;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::text::TextDrawer;

impl TextDrawer {
    pub fn q_draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        font_name: &str,
        text: &str,
        layout_settings: &LayoutSettings,
        color: Color,
        font_size: f32,
    ) -> Result<(), String> {
        let mut layout: Layout<Color> = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(layout_settings);
        let font_index = match self.fonts_id.get(font_name) {
            None => return Err("Font name not existing".to_string()),
            Some(i) => i,
        };
        layout.append(
            &self.fonts,
            &TextStyle::with_user_data(text, font_size, *font_index, color),
        );
        let mut font_texture = FontTexture::new(&self.texture_creator)?;
        font_texture.draw_text(canvas, &self.fonts, layout.glyphs())
    }
}
