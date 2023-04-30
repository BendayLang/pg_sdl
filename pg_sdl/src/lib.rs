#![allow(dead_code, unused_imports, unused_variables)]

pub mod app;
pub mod blocs;
pub mod camera;
pub mod canvas;
pub mod color;
pub mod draw_circle;
pub mod input;
pub mod text;
pub mod utils;
pub mod widgets;

pub mod prelude {
    pub use crate::app::{App, UserApp};
    pub use crate::blocs::{draw_bloc, set_child, Bloc};
    pub use crate::camera::Camera;
    pub use crate::canvas::{fill_background, fill_rect};
    pub use crate::color::{darker, hsv_color, paler, Colors};
    pub use crate::draw_circle::{draw_circle, fill_circle};
    pub use crate::input::Input;
    pub use crate::point;
    pub use crate::rect;
    pub use crate::text::{Text, TextDrawer};
    pub use crate::widgets::Widget;
    pub use crate::widgets::{Button, Orientation, Slider, SliderType};
    pub use fontdue::layout::{HorizontalAlign, VerticalAlign};
    pub use sdl2::{
        self,
        pixels::Color,
        rect::{Point, Rect},
        render::Canvas,
        video::Window,
    };
}
