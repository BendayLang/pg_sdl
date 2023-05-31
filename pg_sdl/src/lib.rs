#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

//! # Getting started
//!
//! ```rust,no_run
//!
//! use pg_sdl::prelude::*;
//!
//! pub fn main() {}
//! ```

pub mod app;
pub mod camera;
pub mod canvas;
pub mod color;
pub mod draw_circle;
pub mod input;
pub mod style;
pub mod text;
pub mod utils;
pub mod vector2;
pub mod widgets;

pub mod prelude {
	pub use crate::app::{App, PgSdl};
	pub use crate::camera::Camera;
	pub use crate::canvas::{fill_background, fill_rect};
	pub use crate::color::{darker, hsv_color, paler, Colors};
	pub use crate::draw_circle::{draw_circle, fill_circle};
	pub use crate::input::Input;
	pub use crate::point;
	pub use crate::rect;
	pub use crate::style::Align;
	pub use crate::text::{TextDrawer, TextStyle};
	pub use crate::widgets::{Button, Orientation, Slider, SliderType, TextInput, TextInputStyle, Widget, Widgets};
	pub use sdl2::{
		self,
		gfx::primitives::DrawRenderer,
		pixels::Color,
		rect::{Point, Rect},
		render::Canvas,
		video::Window,
	};
}
