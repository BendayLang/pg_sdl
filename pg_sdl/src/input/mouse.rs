use sdl2::rect::Point;

use super::KeyState;

pub struct Mouse {
    pub position: Point,
    pub delta: Point,
    left_button_last_release: std::time::Instant,
    pub left_button: KeyState,
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
            left_button: KeyState::Up,
            right_button: KeyState::Up,
            middle_button: KeyState::Up,
            wheel: 0,
            left_button_last_release: std::time::Instant::now(),
        }
    }

    pub fn get_events(&mut self) {
        self.delta = Point::new(0, 0);
        self.wheel = 0;

        self.left_button = match self.left_button {
            KeyState::Pressed => KeyState::Down,
            KeyState::Released => KeyState::Up,
            _ => self.left_button,
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
                sdl2::mouse::MouseButton::Left => self.left_button = KeyState::Pressed,
                sdl2::mouse::MouseButton::Right => self.right_button = KeyState::Pressed,
                sdl2::mouse::MouseButton::Middle => self.middle_button = KeyState::Pressed,
                _ => {}
            },
            Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                sdl2::mouse::MouseButton::Left => {
                    self.left_button_last_release = std::time::Instant::now();
                    self.left_button = KeyState::Released
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
        self.left_button.is_pressed() && self.left_button_last_release.elapsed().as_millis() < Self::TIME_TO_DOUBLE_CLICK
    }
}
