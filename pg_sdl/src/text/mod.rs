mod fonts_init;
mod text;

use fontdue;
use fontdue::layout::{CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign};
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
        text: &Text,
        position: Point,
        width: Option<f32>,
        height: Option<f32>,
        horizontal_align: HorizontalAlign,
        vertical_align: VerticalAlign,
    ) {
        let mut font_texture = FontTexture::new(&self.texture_creator).unwrap();
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        
        if width == None{
            let width = text.text.len() as f32 * text.font_size;
        }
        layout.reset(&LayoutSettings {
            x: position.x as f32,
            y: position.y as f32,
            max_width: width,
            max_height: height,
            horizontal_align,
            vertical_align,
            wrap_style: fontdue::layout::WrapStyle::Word,
            wrap_hard_breaks: false,
            line_height: 1.,
        });
        layout.append(
            &self.fonts,
            &TextStyle::with_user_data(
                &text.text,
                text.font_size,
                text.font_index,
                text.color),
        );
        font_texture.draw_text(canvas, &self.fonts, layout.glyphs()).unwrap();
    }
}
