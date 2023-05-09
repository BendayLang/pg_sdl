use crate::prelude::*;
mod text;
use sdl2::render::TextureQuery;
use std::path::Path;
pub use text::TextStyle;

pub struct TextDrawer {
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ttf_context: sdl2::ttf::Sdl2TtfContext,
}

impl TextDrawer {
    pub fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        TextDrawer {
            texture_creator,
            ttf_context: sdl2::ttf::init().map_err(|e| e.to_string()).unwrap(),
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        position: Point,
        text_style: &TextStyle,
        text: &str,
        align: Align,
    ) {
        let TextStyle {
            // text,
            font_name: font_path,
            font_size,
            font_style,
            color,
        } = text_style;

        // TODO horizontal alignment !!

        let mut font: sdl2::ttf::Font = self
            .ttf_context
            .load_font(Path::new(&font_path), *font_size)
            .unwrap();

        font.set_style(*font_style);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text)
            .blended(*color)
            .map_err(|e| e.to_string())
            .unwrap();

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let size = point!(width, height);

        let target_position = match align {
            Align::TopLeft => position,
            Align::Top => position - point!(size.x / 2, 0),
            Align::TopRight => position - point!(size.x, 0),
            Align::Left => position - point!(0, size.y / 2),
            Align::Center => position - size / 2,
            Align::Right => position - point!(size.x, size.y / 2),
            Align::BottomLeft => position - point!(0, size.y),
            Align::Bottom => position - point!(size.x / 2, size.y),
            Align::BottomRight => position - size,
        };
        let target = rect!(target_position.x, target_position.y, width, height);

        canvas.copy(&texture, None, Some(target)).unwrap();
    }
}
