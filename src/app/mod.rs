use sdl2::{render::Canvas, video::Window};

pub struct App {
    pub input: crate::input::Input,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub fonts: Vec<fontdue::Font>,
}

fn fonts_init() -> Vec<fontdue::Font> {
    let font = include_bytes!("/usr/share/fonts/TTF/VeraBd.ttf") as &[u8];
    let vera_bd = fontdue::Font::from_bytes(font, Default::default()).unwrap();
    let font = include_bytes!("/usr/share/fonts/TTF/VeraIt.ttf") as &[u8];
    let vera_it = fontdue::Font::from_bytes(font, Default::default()).unwrap();
    vec![vera_it, vera_bd]
}

impl App {
    pub fn init(window_title: &str, window_width: u32, window_height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let video_subsystem = sdl_context
            .video()
            .expect("SDL video subsystem could not be initialized");
        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()
            .expect("Window could not be created");
        let canvas = window.into_canvas().build().unwrap();
        App {
            texture_creator: canvas.texture_creator(),
            input: crate::input::Input::new(sdl_context),
            video_subsystem,
            canvas,
            ttf_context,
            fonts: fonts_init(),
        }
    }

    pub fn main_loop<G>(&mut self, update: &mut G)
    where
        G: FnMut(&mut App, f32) -> (),
    {
        'running: loop {
            self.input.get_events();
            if self.input.should_quit
                || self.input.keys_state.esc == crate::input::KeyState::Pressed
            {
                break 'running;
            }

            update(self, 1.0 / 60.0);

            self.canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
