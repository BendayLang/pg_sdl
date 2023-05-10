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
pub use text_input::{TextInput, TextInputStyle};

const HOVER: f32 = 0.94;
const PUSH: f32 = 0.80;

/// A widget is a UI object that can be interacted with to take inputs from the user.
pub trait Widget: AsAny {
    /// Update the widget based on the inputs
    fn update(&mut self, input: &Input, delta: f32, text_drawer: &mut TextDrawer) -> bool;
    /// Draw the widget on the canvas
    fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer);
}

pub struct Widgets(HashMap<String, Box<dyn Widget>>);

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

    pub fn draw(&self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        for widget in self.0.values() {
            widget.draw(canvas, text_drawer);
        }
    }

    pub fn update(&mut self, input: &Input, delta: f32, text_drawer: &mut TextDrawer) -> bool {
        let mut redraw = false;
        for widget in self.0.values_mut() {
            redraw |= widget.update(input, delta, text_drawer);
        }
        redraw
    }

    // TODO: remove this and replace with a macro that right all the code for us
    // and for every widget type

    pub fn get_mut_button(&mut self, name: &str) -> &mut Button {
        self.get_mut(name).unwrap()
    }

    pub fn get_button(&self, name: &str) -> &Button {
        self.get(name).unwrap()
    }

    pub fn get_mut_slider(&mut self, name: &str) -> &mut Slider {
        self.get_mut(name).unwrap()
    }

    pub fn get_slider(&self, name: &str) -> &Slider {
        self.get(name).unwrap()
    }
}
