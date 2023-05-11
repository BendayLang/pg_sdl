use sdl2::rect::Point;
use crate::input::key_state::ChadKeyState;
use std::time::Instant;

use super::KeyState;

pub struct Mouse {
    pub position: Point,
    pub delta: Point,
    left_button_last_release: Instant,
    pub left_button: ChadKeyState,
    pub right_button: KeyState,
    pub middle_button: KeyState,
    pub wheel: i32,
}

impl Mouse {
    const TIME_TO_DOUBLE_CLICK: u128 = 100;

    pub fn new() -> Self {
        Mouse {
            position: Point::new(0, 0),
            delta: Point::new(0, 0),
            left_button: ChadKeyState::Down { pressed_time: Instant::now() }, // TODO idee de merde ?
            right_button: KeyState::Up,
            middle_button: KeyState::Up,
            wheel: 0,
            left_button_last_release: Instant::now(),
        }
    }

    // TODO rename update ???
    pub fn get_events(&mut self) {
        self.delta = Point::new(0, 0);
        self.wheel = 0;

        self.left_button = match &self.left_button {
            ChadKeyState::Pressed => ChadKeyState::Down { pressed_time: Instant::now() },
            ChadKeyState::Released { last_state } => ChadKeyState::Up { released_time: Instant::now(), last_state: Box::new(self.left_button.clone()) },
            _ => self.left_button.clone()
        };
        self.right_button = match self.right_button {
            KeyState::Pressed => KeyState::Down,
            KeyState::Released => KeyState::Up,
            _ => self.right_button,
        };
        self.middle_button = match self.middle_button {
            KeyState::Pressed => KeyState::Down,
            KeyState::Released => KeyState::Up,
            _ => self.middle_button,
        };
    }

    pub fn get_event(&mut self, event: sdl2::event::Event) {
        use sdl2::event::Event;
        match event {
            Event::MouseMotion {
                x, y, xrel, yrel, ..
            } => {
                self.position = Point::new(x, y);
                self.delta = Point::new(xrel, yrel);
            }
            Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                sdl2::mouse::MouseButton::Left => self.left_button.press(),
                sdl2::mouse::MouseButton::Right => self.right_button = KeyState::Pressed,
                sdl2::mouse::MouseButton::Middle => self.middle_button = KeyState::Pressed,
                _ => {}
            },
            Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                sdl2::mouse::MouseButton::Left => {
                    self.left_button_last_release = Instant::now();
                    self.left_button = ChadKeyState::Released { last_state: Box::new(self.left_button.clone()) }
                }
                sdl2::mouse::MouseButton::Right => self.right_button = KeyState::Released,
                sdl2::mouse::MouseButton::Middle => self.middle_button = KeyState::Released,
                _ => {}
            },
            Event::MouseWheel { y, .. } => {
                self.wheel = y;
            }
            _ => {}
        }
    }

    pub fn left_button_double_clicked(&self) -> bool {
        self.left_button.is_double_pressed()
        // self.left_button.is_pressed() && self.left_button_last_release.elapsed().as_millis() < Self::TIME_TO_DOUBLE_CLICK
    }
}
