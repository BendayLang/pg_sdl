use crate::prelude::*;
use std::collections::HashMap;
mod text;
use sdl2::render::TextureQuery;
use std::path::Path;
pub use text::Text;

pub struct TextDrawer {
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    fonts: HashMap<(String, usize), sdl2::ttf::Font<'static, 'static>>,
}

impl TextDrawer {
    pub fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        TextDrawer {
            texture_creator,
            fonts: HashMap::new(),
        }
    }

    // // TODO use this to call the [draw_text] fn with a font name instead of its index
    // fn font_index_from_name(&self, name: &str) -> Option<&usize> {
    //     self.fonts_id.get(name)
    // }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        position: Point,
        text: &str,
        font_path: &str,
        font_size: u16,
        font_style: sdl2::ttf::FontStyle,
        color: Color,
    ) {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        // let font_name = format!(
        //     "C:/Users/arnol/PycharmProjects/LibTests/venv/Lib/site-packages/kivy/data/fonts/{}.ttf",
        //     font_name
        // );
        let mut font = ttf_context
            .load_font(Path::new(font_path), font_size)
            .unwrap();
        font.set_style(font_style);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let target = rect!(
            position.x - (width / 2) as i32,
            position.y - (height / 2) as i32,
            width,
            height
        );

        canvas.copy(&texture, None, Some(target)).unwrap();
    }
}
