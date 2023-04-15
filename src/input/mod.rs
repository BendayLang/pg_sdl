use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    surface::SurfaceRef,
    ttf::Font,
    video::{Window, WindowContext},
    EventPump,
};

mod key_state;
pub use key_state::{KeyState, KeysState};

pub struct Input {
    event_pump: EventPump,
    pub should_quit: bool,
    pub keys_state: KeysState,
}

impl Input {
    /// can crash
    pub fn new(sdl_context: sdl2::Sdl) -> Self {
        Input {
            event_pump: sdl_context.event_pump().unwrap(),
            should_quit: false,
            keys_state: KeysState::new(),
        }
    }

    /// should be called every frame
    pub fn get_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.should_quit = true,
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    if let Some(keycode) = keycode {
                        self.keys_state.set_key_state(keycode, true);
                    }
                }
                Event::KeyUp {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    if let Some(keycode) = keycode {
                        self.keys_state.set_key_state(keycode, false);
                    }
                }
                _ => {}
            }
        }
    }
}
