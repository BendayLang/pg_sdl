pub mod button;
pub mod slider;
pub mod text_input;

use crate::input::Input;
use crate::text::TextDrawer;
use as_any::{AsAny, Downcast};
use sdl2::render::Canvas;
use sdl2::video::Window;
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

#[macro_export]
/// Get a widget mutable reference from the widgets hashmap.
/// And panic if the widget is not found or if the type is not correct.
macro_rules! get_mut_widget {
    ($widgets:expr, $name:expr, $type:ty) => {
        $widgets.get_mut::<$type>($name).unwrap()
    };
}

#[macro_export]
/// Get a widget reference from the widgets hashmap.
/// And panic if the widget is not found or if the type is not correct.
macro_rules! get_widget {
    ($widgets:expr, $name:expr, $type:ty) => {
        $widgets.get::<$type>($name).unwrap()
    };
}

#[macro_export]
macro_rules! get_button {
    ($widgets:expr, $name:expr) => {
        $widgets.get::<Button>($name).unwrap()
    };
}

#[macro_export]
macro_rules! get_button_mut {
    ($widgets:expr, $name:expr) => {
        $widgets.get_mut::<Button>($name).unwrap()
    };
}

#[macro_export]
macro_rules! get_slider {
    ($widgets:expr, $name:expr) => {
        $widgets.get::<Slider>($name).unwrap()
    };
}

#[macro_export]
macro_rules! get_slider_mut {
    ($widgets:expr, $name:expr) => {
        $widgets.get_mut::<Slider>($name).unwrap()
    };
}
