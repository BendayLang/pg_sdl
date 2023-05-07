mod key_state;
mod mouse;

pub use key_state::{KeyState, KeysState};

pub struct Input {
    event_pump: sdl2::EventPump,
    pub window_closed: bool,
    pub keys_state: KeysState,
    pub mouse: mouse::Mouse,
    pub last_char: Option<char>,
}

impl Input {
    /// can crash
    pub fn new(sdl_context: sdl2::Sdl) -> Self {
        Input {
            event_pump: sdl_context.event_pump().unwrap(),
            window_closed: false,
            keys_state: KeysState::new(),
            mouse: mouse::Mouse::new(),
            last_char: None,
        }
    }

    /// should be called every frame
    pub fn get_events(&mut self) {
        self.last_char = None;

        for key_state in self.keys_state.as_mut_array() {
            key_state.update()
        }

        self.mouse.get_events();

        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event;
            self.mouse.get_event(event.clone());
            match event {
                Event::Quit { .. } => self.window_closed = true,
                Event::KeyDown { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        self.keys_state.press_key(keycode);
                    }

                    // sks(keycode, true);

                    if let Some(keycode) = keycode {
                        let c = keycode as u8 as char;
                        if c.is_ascii_alphanumeric()
                            || c.is_ascii_punctuation()
                            || c.is_ascii_whitespace()
                        {
                            self.last_char = Some(keycode as u8 as char);
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        self.keys_state.release_key(keycode);
                    }
                }
                _ => {}
            }
        }
    }
}
