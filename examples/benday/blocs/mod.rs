use crate::rect;
use crate::text::TextDrawer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;
use std::fmt::Display;

/// A bloc represents a piece of code that can be executed.
///
/// If the bloc has a parent, the rect position is relative to the parent.
pub struct Bloc {
    color: Color,
    rect: Rect,
    corner_radius: Option<u32>,
    pub parent: Option<u32>,
    pub child: Option<u32>,
}

impl Display for Bloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent = if let Some(parent) = self.parent {
            format!("{}", parent)
        } else {
            "N".to_string()
        };
        let child = if let Some(child) = self.child {
            format!("{}", child)
        } else {
            "N".to_string()
        };
        write!(f, "Bloc( parent:{} child:{} )", parent, child)
    }
}

impl Bloc {
    pub fn new(color: Color, rect: Rect) -> Self {
        Self {
            color,
            rect,
            corner_radius: None,
            parent: None,
            child: None,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        // canvas.set_draw_color(self.color);
        // fill_rect(canvas, self.rect, self.corner_radius);
        // canvas.set_draw_color(Color::BLACK);
        // draw_rect(canvas, self.rect);

        let texture_creator = canvas.texture_creator();

        let mut surface = Surface::new(
            self.rect.width(),
            self.rect.height(),
            texture_creator.default_pixel_format(),
        )
        .unwrap();

        surface
            .fill_rect(
                rect!(0, 0, self.rect.width(), self.rect.height()),
                self.color,
            )
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.copy(&texture, None, Some(self.rect)).unwrap();
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.rect.size()
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        self.rect.resize(size.0, size.1);
    }

    pub fn move_by(&mut self, point: Point) {
        self.rect.reposition(self.rect.top_left() + point);
    }

    pub fn set_position(&mut self, position: (i32, i32)) {
        self.rect.reposition(position);
    }

    pub fn collide(&self, point: Point) -> bool {
        self.rect.contains_point(point)
    }

    pub fn collide_bloc(&self, bloc: &Bloc) -> bool {
        self.rect.has_intersection(bloc.rect)
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }
}

pub fn draw_bloc<'a>(
    bloc: &Bloc,
    blocs: &HashMap<u32, Bloc>,
    canvas: &mut Canvas<Window>,
    text_drawer: &mut TextDrawer,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Surface<'a> {
    let mut surface = Surface::new(
        bloc.rect.width(),
        bloc.rect.height(),
        texture_creator.default_pixel_format(),
    )
    .unwrap();
    surface
        .fill_rect(
            rect!(0, 0, bloc.rect.width(), bloc.rect.height()),
            bloc.color,
        )
        .unwrap();

    if let Some(child_id) = bloc.child {
        let bloc = blocs.get(&child_id).unwrap();
        let child_surface = draw_bloc(bloc, blocs, canvas, text_drawer, texture_creator);
        child_surface
            .blit(None, surface.as_mut(), Some(bloc.rect))
            .unwrap();
    }

    surface
}

pub fn set_child(child_id: u32, parent_id: u32, blocs: &mut HashMap<u32, Bloc>) {
    let child_size = blocs.get(&child_id).unwrap().get_size();
    let (child_width, child_height) = child_size;
    {
        let parent_bloc = blocs.get_mut(&parent_id).unwrap();
        parent_bloc.child = Some(child_id);

        parent_bloc.set_size((
            parent_bloc.rect.width() + child_width,
            parent_bloc.rect.height() + child_height,
        ));
    }

    let parent_size = blocs.get(&parent_id).unwrap().get_size();
    let (parent_width, parent_height) = parent_size;
    {
        let child_bloc = blocs.get_mut(&child_id).unwrap();
        child_bloc.parent = Some(parent_id);

        child_bloc.set_position((
            (parent_width as i32 - child_width as i32) / 2,
            (parent_height as i32 - child_height as i32) / 2,
        ));
    }
}
