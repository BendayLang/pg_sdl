use pg_sdl::color::Colors;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode::V;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug)]
/// Particles are objects that have mass, position, velocity, and respond to forces,
///
/// but that have no spatial extent.
pub struct Particle {
    mass: f32,
    position: Vec2,
    velocity: Vec2,
    force_accumulator: Vec<Vec2>,
    radius: f32,
    color: Color,
}
impl Particle {
    pub fn new(mass: f32, position: Vec2, radius: f32, color: Color) -> Self {
        Self {
            mass,
            position,
            velocity: Vec2::ZERO,
            force_accumulator: Vec::new(),
            radius,
            color,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.force_accumulator.push(force);
    }

    pub fn get_force(&self) -> Vec2 {
        self.force_accumulator.clone().into_iter().sum()
    }

    pub fn collide_point(&self, point: Vec2) -> bool {
        (self.position - point).length() < self.radius
    }

    pub fn update(&mut self, delta: f32) {
        let force = self.get_force();
        // Euler integration
        // let acceleration = self.force_accumulator / self.mass;

        // Runge-Kutta integration
        let k1 = force / self.mass;
        let k2 = (force + k1 * delta / 2.0) / self.mass;
        let k3 = (force + k2 * delta / 2.0) / self.mass;
        let k4 = (force + k3 * delta) / self.mass;
        let acceleration = (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

        self.velocity += acceleration * delta;
        self.position += self.velocity * delta;
    }

    pub fn clear_force_accumulator(&mut self) {
        self.force_accumulator.clear();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if self.radius == 0.0 {
            return;
        }
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
        .unwrap();
    }

    pub fn draw_forces(&self, canvas: &mut Canvas<Window>, scaler: f32) {
        for (i, force) in self.force_accumulator.iter().enumerate() {
            let color = if i == self.force_accumulator.len() - 1 {
                Colors::VIOLET
            } else {
                Colors::LIGHT_YELLOW
            };
            draw_arrow(
                canvas,
                color,
                self.position,
                self.position + *force * scaler,
                8.0,
            );
        }
    }
}

/// Draw an arrow from start to end with the head at the end.
fn draw_arrow(canvas: &mut Canvas<Window>, color: Color, start: Vec2, end: Vec2, width: f32) {
    if start == end {
        return;
    }
    let x_dir = end - start;
    let y_dir = x_dir.perpendicular() * width / 2.0;
    let transform = |v: Vec2| start + v.linear_transform(x_dir, y_dir);

    let head_back: f32 = 1.0 - 3.0 * width / x_dir.length();

    let mut points = Vec::from([
        Vec2::new(head_back, -1.0),
        Vec2::new(head_back, -3.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(head_back, 3.0),
        Vec2::new(head_back, 1.0),
    ]);
    if x_dir.length() > 3.0 * width {
        points.append(&mut Vec::from([Vec2::new(0.0, 1.0), Vec2::new(0.0, -1.0)]));
    }
    points.iter_mut().for_each(|v| *v = transform(*v));
    let points_x: Vec<i16> = points.iter().map(|v| v.x as i16).collect();
    let points_y: Vec<i16> = points.iter().map(|v| v.y as i16).collect();

    DrawRenderer::filled_polygon(canvas, &points_x, &points_y, color).unwrap();
    DrawRenderer::polygon(canvas, &points_x, &points_y, Colors::BLACK).unwrap();
}
