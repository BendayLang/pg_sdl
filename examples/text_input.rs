use pg_sdl::prelude::*;

fn main() {
    let mut app = PgSdl::init("Text input", 800, 600, Some(60), true, Color::GRAY);
    app.add_widget(
        "input",
        Box::new(TextInput::new(
            Some(TextInputStyle {
                rect: Rect::new(100, 100, 100, 20),
                ..TextInputStyle::default()
            }),
            Some("Hello".to_string()),
        )),
    );
    let mut app_state = AppState {};
    app.run(&mut app_state);
}

struct AppState {}

impl App for AppState {
    fn update(&mut self, _delta: f32, _input: &Input, _widgets: &mut Widgets) -> bool {
        false
    }
    fn draw(&mut self, _canvas: &mut Canvas<Window>, _text_drawer: &mut TextDrawer) {}
}
