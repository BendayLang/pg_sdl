use pg_sdl::prelude::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const INPUT_WIDTH: u32 = 120;

fn main() {
    let mut app = PgSdl::init(
        "Text input",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        Some(60),
        false,
        Color::GRAY,
    );
    app.add_widget(
        "login input",
        Box::new(TextInput::new(
            Rect::new(
                WINDOW_WIDTH as i32 / 2 - INPUT_WIDTH as i32 / 2,
                100,
                INPUT_WIDTH,
                30,
            ),
            None,
            Some("Thierry".to_string()),
        )),
    )
    .add_widget(
        "password input",
        Box::new(TextInput::new(
            Rect::new(
                WINDOW_WIDTH as i32 / 2 - INPUT_WIDTH as i32 / 2,
                150,
                INPUT_WIDTH,
                30,
            ),
            None,
            None,
        )),
    )
    .add_widget(
        "login button",
        Box::new(Button::new(
            Colors::ROYAL_BLUE,
            rect!(
                WINDOW_WIDTH as i32 / 2 - INPUT_WIDTH as i32 / 2,
                200,
                INPUT_WIDTH,
                30
            ),
            None,
            TextStyle::default(),
            "Login".to_string(),
        )),
    );
    let mut app_state = AppState {};
    app.run(&mut app_state);
}

struct AppState {}

impl App for AppState {
    fn update(&mut self, _delta: f32, _input: &Input, widgets: &mut Widgets) -> bool {
        if widgets.get_button("login button").state.is_pressed() {
            println!(
                "Login: {}, password: {}",
                widgets.get::<TextInput>("login input").unwrap().content,
                widgets.get::<TextInput>("password input").unwrap().content
            );
        }
        false
    }
    fn draw(&mut self, _canvas: &mut Canvas<Window>, _text_drawer: &mut TextDrawer) {}
}
