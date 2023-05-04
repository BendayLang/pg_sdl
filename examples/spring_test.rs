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
    width: f32,
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
            width: radius,
            color,
        }
    }

    fn direction(&self) -> Vec2 {
        (self.end - self.start).normalized()
    }

    fn length(&self) -> f32 {
        (self.end - self.start).length()
    }

    fn get_force(&self, mass: &Mass) -> Vec2 {
        let force = -self.k * (self.length() - self.default_length);
        let damping = -self.b * mass.velocity.dot(self.direction());
        self.direction() * (force + damping)
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let x_dir = self.end - self.start - self.width;
        let y_dir = x_dir.normal() * self.width;
        let transform = |v: Vec2| {
            self.start - y_dir / 2.0
                + x_dir.with_length(self.width / 2.0)
                + v.linear_transform(x_dir, y_dir)
        };

        let draw_thick_line = |start: Vec2, end: Vec2, width: u8, color: Color| {
            DrawRenderer::thick_line(
                canvas,
                start.x as i16,
                start.y as i16,
                end.x as i16,
                end.y as i16,
                width,
                color,
            )
            .unwrap();
            let p = end - start;
            let q = p.normal() * width as f32 / 2.0;
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
        };

        let draw_rect = |start: Vec2, end: Vec2, width: u8, color: Color| {
            draw_thick_line(transform(start), transform(end), width, color);
            let start1 = transform(start + Vec2::new_x(-(width as f32) / 2.0 / x_dir.length()));
            let end1 = transform(start + Vec2::new_x((width as f32) / 2.0 / x_dir.length()));
            DrawRenderer::line(
                canvas,
                start1.x as i16,
                start1.y as i16,
                end1.x as i16,
                end1.y as i16,
                Colors::BLACK,
            )
            .unwrap();
            let start2 = transform(end + Vec2::new_x(-(width as f32) / 2.0 / x_dir.length()));
            let end2 = transform(end + Vec2::new_x((width as f32) / 2.0 / x_dir.length()));
            DrawRenderer::line(
                canvas,
                start2.x as i16,
                start2.y as i16,
                end2.x as i16,
                end2.y as i16,
                Colors::BLACK,
            )
            .unwrap();
        };

        let draw_circle = |center: Vec2, radius: i16, color: Color| {
            DrawRenderer::filled_circle(canvas, center.x as i16, center.y as i16, radius, color)
                .unwrap();
            DrawRenderer::circle(
                canvas,
                center.x as i16,
                center.y as i16,
                radius,
                Colors::BLACK,
            )
            .unwrap();
        };

        let draw_rounded_thick_line = |start: Vec2, end: Vec2, width: u8, color: Color| {
            draw_circle(start, width as i16 / 2 + 1, color);
            draw_circle(end, width as i16 / 2 + 1, color);
            draw_thick_line(start, end, width, color);
        };

        /*
        let start = transform(Vec2::new(0.0, 0.5));
        let end = transform(Vec2::new(1.0, 0.5));
        DrawRenderer::thick_line(
            canvas,
            start.x as i16,
            start.y as i16,
            end.x as i16,
            end.y as i16,
            self.width as u8,
            Colors::WHITE,
        )
        .unwrap();
         */

        let start = self.start + y_dir.with_length(0.0);
        draw_circle(start, (self.width / 4.0) as i16, self.color);
        draw_thick_line(
            self.start,
            transform(Vec2::new(0.0, 0.5)),
            (self.width / 2.0) as u8,
            self.color,
        );

        draw_circle(self.end, (self.width / 4.0) as i16, self.color);
        draw_thick_line(
            transform(Vec2::new(1.0, 0.5)),
            self.end,
            (self.width / 2.0) as u8,
            self.color,
        );

        let n = 4;
        let dp = 0.5 / n as f32;

        (0..n).for_each(|i| {
            let p = i as f32 / n as f32;
            draw_thick_line(
                transform(Vec2::new(p, 0.0)),
                transform(Vec2::new(p + dp, 1.0)),
                (self.width / 4.0) as u8,
                darker(self.color, 0.8),
            );
        });
        (0..n).for_each(|i| {
            let p = i as f32 / n as f32;
            draw_rounded_thick_line(
                transform(Vec2::new(p + dp, 1.0)),
                transform(Vec2::new(p + dp * 2.0, 0.0)),
                (self.width / 3.5) as u8,
                self.color,
            );
        });

        draw_rect(
            Vec2::new(0.0, -0.15),
            Vec2::new(0.0, 1.15),
            (self.width / 3.0) as u8,
            self.color,
        );
        draw_rect(
            Vec2::new(1.0, -0.15),
            Vec2::new(1.0, 1.15),
            (self.width / 3.0) as u8,
            self.color,
        );
    }
}

struct Mass {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    radius: f32,
    color: Color,
    force_accumulator: Vec2,
}
impl Mass {
    fn new(position: Vec2, velocity: Vec2, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            mass,
            radius,
            color,
            force_accumulator: Vec2::ZERO,
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        self.force_accumulator += force;
    }

    fn update(&mut self, delta: f32) {
        let acceleration = self.force_accumulator / self.mass;
        self.velocity += acceleration * delta;
        self.position += self.velocity * delta;

        self.force_accumulator = Vec2::ZERO;
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
    selected: Option<Spring>,
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

        self.mass.apply_force(Vec2::new_y(9.81) * self.mass.mass);
        self.mass.apply_force(self.spring.get_force(&self.mass));

        if input.mouse.left_button.is_pressed() {
            let mouse_position = Vec2::from(input.mouse.position);
            if self.mass.collide_point(mouse_position) {
                self.spring.end = mouse_position;
                self.selected = Some(Spring::new(
                    mouse_position,
                    self.mass.position,
                    1.0,
                    2.0,
                    20.0,
                    Colors::GREEN,
                ));
            }
        } else if input.mouse.left_button.is_released() {
            self.selected = None;
        }

        if let Some(spring) = &mut self.selected {
            let mouse_position = Vec2::from(input.mouse.position);
            spring.start = mouse_position;
            spring.end = self.mass.position;
            self.mass.apply_force(spring.get_force(&self.mass));
        }

        self.mass.update(delta * self.sliders[0].get_value() * 10.0);
        self.spring.end = self.mass.position;

        changed = true;
        changed
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        self.widgets()
            .iter()
            .for_each(|widget| widget.draw(canvas, text_drawer));

        self.mass.draw(canvas);
        self.spring.draw(canvas);
        if let Some(spring) = &mut self.selected {
            spring.draw(canvas);
        }
    }
}

fn main() {
    let my_app = &mut MyApp {
        buttons: vec![Button::new(
            Colors::ORANGE,
            rect!(300, 35, 120, 50),
            Some(9),
            Some(Text::new("Start".to_string(), 16, None)),
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
            Vec2::new(600.0, 450.0),
            0.2,
            0.01,
            40.0,
            Colors::BEIGE,
        ),
        mass: Mass::new(
            Vec2::new(600.0, 400.0),
            Vec2::ZERO,
            1.0,
            25.0,
            Colors::RED_ORANGE,
        ),
        selected: None,
    };

    let mut app: App = App::init("Spring test", 1200, 720, Some(60.0), true, Colors::SKY_BLUE);
    app.run(my_app);
}
