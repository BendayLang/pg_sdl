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
        let w: u8 = 1;
        let l: u8 = 10;

        let dir = self.end - self.start;
        let per = dir.rotated_deg(90.0).normalized() * self.radius / 2.0;
        let adv = |(x, y): (f32, f32)| self.start + dir * x + per * y;

        [
            (adv((0.1, -1.0)), adv((0.1, 1.0))),
            (adv((0.9, -1.0)), adv((0.9, 1.0))),
        ]
        .iter()
        .for_each(|(start, end)| {
            DrawRenderer::thick_line(
                canvas,
                start.x as i16,
                start.y as i16,
                end.x as i16,
                end.y as i16,
                l as u8,
                Colors::BLACK,
            )
            .unwrap();
            DrawRenderer::thick_line(
                canvas,
                start.x as i16,
                start.y as i16,
                end.x as i16,
                end.y as i16,
                l - 2 * w,
                self.color,
            )
            .unwrap();
        })
    }
}

pub struct MyApp {
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
    springs: Vec<Spring>,
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

        self.springs.iter_mut().for_each(|spring| {
            spring.end.rotate_around_deg(1.0, spring.start);

            let angle = spring.end.angle_to(spring.start);
            spring.end = Vec2::from_polar(
                spring.default_length * (1.0 + 0.2 * (5.0 * angle).sin()),
                angle,
            ) + spring.start;
        });

        true
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, text_drawer: &mut TextDrawer) {
        self.widgets()
            .iter()
            .for_each(|widget| widget.draw(canvas, text_drawer));

        self.springs.iter().for_each(|spring| {
            spring.draw(canvas);
            DrawRenderer::filled_circle(
                canvas,
                spring.start.x as i16,
                spring.start.y as i16,
                4,
                Colors::BLACK,
            )
            .unwrap();
            DrawRenderer::filled_circle(
                canvas,
                spring.end.x as i16,
                spring.end.y as i16,
                4,
                Colors::BLACK,
            )
            .unwrap();
        });
    }
}

fn main() {
    let my_app = &mut MyApp {
        buttons: vec![Button::new(
            Colors::ROYAL_BLUE,
            rect!(200, 50, 120, 50),
            Some(9),
            Some(Text::new("Pause".to_string(), 16, None)),
        )],
        sliders: vec![Slider::new(
            Colors::ORANGE,
            rect!(100, 80, 30, 150),
            Some(20),
            SliderType::Continuous {
                default_value: 0.25,
                display: Some(Box::new(|value| format!("{:.2}", value * 100.0 - 50.0))),
            },
        )],

        springs: vec![Spring::new(
            Vec2::new(600.0, 350.0),
            Vec2::new(750.0, 350.0),
            0.5,
            0.1,
            40.0,
            Colors::RED,
        )],
    };

    let mut app: App = App::init("Spring test", 1200, 720, Some(60.0), true, Colors::SKY_BLUE);
    app.run(my_app);
}
