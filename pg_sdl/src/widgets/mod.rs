pub(crate) mod button;
pub(crate) mod slider;

use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::Input;
use crate::text::TextDrawer;

const HOVER: f32 = 0.94;
const PUSH: f32 = 0.80;

/// A widget is a UI object that can be interacted with to take inputs from the user.
pub trait Widget {
	/// Update the widget based on the inputs
	fn update(&mut self, input: &Input, delta: f32) -> bool;
	/// Draw the widget on the canvas
	fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer);
}
