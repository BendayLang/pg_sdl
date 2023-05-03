use pg_sdl::prelude::*;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;

struct Spring {
    start: Vec2,
    end: Vec2,
    /// the force constant of the spring
    k: f32,
    /// the damping of the spring
    b: f32,
    default_length: f32,
    radius: f32,
    color: Color,
}

impl Spring {
    fn new(start: Vec2, end: Vec2, k: f32, b: f32, radius: f32, color: Color) -> Self {
        Self {
            default_length: (start - end).length(),
            start,
            end,
            k,
            b,
            radius,
            color,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let w: u8 = 8;

        let x_dir = self.end - self.start;
        let y_dir = x_dir.normal() * self.radius / 2.0;
        let transform = |v: Vec2| self.start + v.linear_transform(x_dir, y_dir);

        [
            ((0.3, -1.0), (0.2, 1.0)),
            ((0.5, -1.0), (0.4, 1.0)),
            ((0.7, -1.0), (0.6, 1.0)),
            ((0.9, -1.0), (0.8, 1.0)),
            ((0.1, -1.0), (0.2, 1.0)),
            ((0.3, -1.0), (0.4, 1.0)),
            ((0.5, -1.0), (0.6, 1.0)),
            ((0.7, -1.0), (0.8, 1.0)),
            ((0.0, 0.0), (0.1, 0.0)),
            ((0.1, -1.0), (0.1, 1.0)),
            ((0.9, 0.0), (1.0, 0.0)),
            ((0.9, -1.0), (0.9, 1.0)),
        ]
        .iter()
        .for_each(|(start_p, end_p)| {
            let start = transform(Vec2::from(*start_p));
            let end = transform(Vec2::from(*end_p));
            DrawRenderer::thick_line(
                canvas,
                start.x as i16,
                start.y as i16,
                end.x as i16,
                end.y as i16,
                w,
                self.color,
            )
            .unwrap();
            let p = end - start;
            let q = p.normal() * w as f32 / 2.0;
            DrawRenderer::line(
                canvas,
                (start.x + q.x) as i16,
                (start.y + q.y) as i16,
                (end.x + q.x) as i16,
                (end.y + q.y) as i16,
                Colors::BLACK,
            )
            .unwrap();
            DrawRenderer::line(
                canvas,
                (start.x - q.x) as i16,
                (start.y - q.y) as i16,
                (end.x - q.x) as i16,
                (end.y - q.y) as i16,
                Colors::BLACK,
            )
            .unwrap();
        })
    }
}

struct Mass {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    radius: f32,
    color: Color,
}
impl Mass {
    fn new(position: Vec2, velocity: Vec2, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            mass,
            radius,
            color,
        }
    }

    fn update(&mut self, delta: f32, force: Vec2) {
        let acceleration = force / self.mass;
        self.velocity += acceleration * delta;
        self.position += self.velocity * delta;
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        DrawRenderer::filled_circle(
            canvas,
            self.position.x as i16,
            self.position.y as i16,
            self.radius as i16,
            self.color,
        )
        .unwrap();
        DrawRenderer::circle(
            canvas,
            self.position.x as i16,
            self.position.y as i16,
            self.radius as i16,
            Colors::BLACK,
        )
        .unwrap()
    }

    fn collide_point(&self, point: Vec2) -> bool {
        (self.position - point).length() < self.radius
    }
}

pub struct MyApp {
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
    spring: Spring,
    mass: Mass,
    selected: bool,
}

impl MyApp {
    fn widgets(&mut self) -> Vec<&mut dyn Widget> {
        self.buttons
            .iter_mut()
            .map(|button| button as &mut dyn Widget)
            .chain(
                self.sliders
                    .iter_mut()
                    .map(|slider| slider as &mut dyn Widget),
            )
            .collect()
    }
}

impl UserApp for MyApp {
    fn update(&mut self, delta: f32, input: &Input) -> bool {
        let mut changed = false;
        changed |= self
            .widgets()
            .iter_mut()
            .any(|widget| widget.update(&input, delta));

        if self.buttons[0].state.is_pressed() {
            if self.sliders[0].get_value() == 0.0 {
                self.sliders[0].set_value(1.0);
                self.buttons[0].set_text("Stop".to_string());
            } else {
                self.sliders[0].set_value(0.0);
                self.buttons[0].set_text("Start".to_string());
            }
        }

        let gravity = Vec2::new_y(9.81) * self.mass.mass;
        let spring_force = (self.spring.end - self.spring.start).normalized()
            * (self.spring.k
                * (self.spring.default_length - (self.spring.end - self.spring.start).length())
                - self.mass.velocity.dot(self.spring.end - self.spring.start) * self.spring.b);
        self.mass.update(
            delta * self.sliders[0].get_value() * 10.0,
            gravity + spring_force,
        );
        self.spring.end = self.mass.position;

        if input.mouse.left_button.is_pressed() {
            if self.mass.collide_point(Vec2::from(input.mouse.position)) {
                self.selected = true;
            }
        } else if input.mouse.left_button.is_released() {
            self.selected = false;
        }

        if self.selected {
            self.mass.position = Vec2::from(input.mouse.position);
            self.mass.velocity = Vec2::ZERO;
        }

        changed = true;
        changed
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        self.widgets()
            .iter()
            .for_each(|widget| widget.draw(canvas, text_drawer));

        self.mass.draw(canvas);
        self.spring.draw(canvas);
    }
}

fn main() {
    let my_app = &mut MyApp {
        buttons: vec![Button::new(
            Colors::ORANGE,
            rect!(300, 35, 120, 50),
            Some(9),
            Some(Text::new("Pause".to_string(), 16, None)),
        )],
        sliders: vec![Slider::new(
            Colors::ORANGE,
            rect!(500, 50, 200, 30),
            Some(20),
            SliderType::Continuous {
                default_value: 0.0,
                display: Some(Box::new(|value| format!("{:.2}", value))),
            },
        )],

        spring: Spring::new(
            Vec2::new(600.0, 300.0),
            Vec2::new(750.0, 300.0),
            0.2,
            0.001,
            40.0,
            Colors::BEIGE,
        ),
        mass: Mass::new(
            Vec2::new(750.0, 300.0),
            Vec2::ZERO,
            1.0,
            20.0,
            Colors::RED_ORANGE,
        ),
        selected: false,
    };

    let mut app: App = App::init(
        "Spring test",
        1200,
        720,
        Some(100.0),
        true,
        Colors::SKY_BLUE,
    );
    app.run(my_app);
}
