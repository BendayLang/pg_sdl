mod fonts_init;
mod text;

use fontdue;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue_sdl2::FontTexture;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
pub use text::Text;


pub struct TextDrawer {
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub fonts: Vec<fontdue::Font>,
}

impl TextDrawer {
    pub fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        TextDrawer {
            texture_creator,
            fonts: fonts_init::fonts_init(),
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        font_index: usize,
        text: &str,
        position: Point,
        font_size: f32,
        color: Color,
    ) {
        let mut font_texture = FontTexture::new(&self.texture_creator).unwrap();
        println!("{}",  font_texture.texture.query().width);
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: position.x as f32,
            y: position.y as f32,
            ..Default::default()
        });
        layout.append(
            &self.fonts,
            &TextStyle::with_user_data(text, font_size, font_index, color),
        );
        println!("{}",  font_texture.texture.query().width);
        font_texture
            .draw_text(canvas, &self.fonts, layout.glyphs())
            .unwrap();
    }
}
