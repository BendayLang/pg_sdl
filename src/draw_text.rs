use crate::app::App;
use fontdue;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue_sdl2::FontTexture;
use sdl2::pixels::Color;

pub fn draw_text(app: &mut App, font_index: usize, text: &str, x: f32, y: f32, font_size: f32) {
    let mut font_texture = FontTexture::new(&app.texture_creator).unwrap();
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        x,
        y,
        ..Default::default()
    });
    layout.append(
        &app.fonts,
        &TextStyle::with_user_data(text, font_size, font_index, Color::RGB(0xFF, 0xFF, 0)),
    );
    font_texture
        .draw_text(&mut app.canvas, &app.fonts, layout.glyphs())
        .unwrap();
}
