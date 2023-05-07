pub mod button;
pub mod slider;

use crate::input::Input;
use crate::text::TextDrawer;
use as_any::{AsAny, Downcast};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::any::Any;
use std::collections::HashMap;

pub use button::Button;
pub use slider::Orientation;
pub use slider::Slider;
pub use slider::SliderType;

const HOVER: f32 = 0.94;
const PUSH: f32 = 0.80;

/// A widget is a UI object that can be interacted with to take inputs from the user.
pub trait Widget: AsAny {
    /// Update the widget based on the inputs
    fn update(&mut self, input: &Input, delta: f32) -> bool;
    /// Draw the widget on the canvas
    fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer);
}

pub struct Widgets(pub HashMap<String, Box<dyn Widget>>);

impl Widgets {
    pub fn new() -> Self {
        Widgets(HashMap::new())
    }

    pub fn add(&mut self, name: &str, widget: Box<dyn Widget>) {
        self.0.insert(name.to_string(), widget);
    }

    pub fn get<T: Widget>(&self, name: &str) -> Option<&T> {
        self.0
            .get(name)
            .and_then(|w| w.as_ref().downcast_ref::<T>())
    }

    pub fn get_mut<T: Widget>(&mut self, name: &str) -> Option<&mut T> {
        self.0
            .get_mut(name)
            .and_then(|w| w.as_mut().downcast_mut::<T>())
    }
}
