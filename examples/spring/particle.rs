use pg_sdl::color::Colors;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// A particle is the most basic element of the physics engine.
///
/// It has a position, a velocity and a mass.
pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    radius: f32,
    color: Color,
    force_accumulator: Vec2,
    fixed: bool,
}
impl Particle {
    pub fn new(position: Vec2, mass: f32, radius: f32, color: Color, fixed: bool) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            mass,
            radius,
            color,
            force_accumulator: Vec2::ZERO,
            fixed,
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
        if self.fixed {
            return;
        }
        self.force_accumulator += force;
    }

    pub fn collide_point(&self, point: Vec2) -> bool {
        (self.position - point).length() < self.radius
    }

    pub fn update(&mut self, delta: f32) {
        if self.fixed {
            return;
        }

        // Euler integration
        // let acceleration = self.force_accumulator / self.mass;

        // Runge-Kutta integration
        let k1 = self.force_accumulator / self.mass;
        let k2 = (self.force_accumulator + k1 * delta / 2.0) / self.mass;
        let k3 = (self.force_accumulator + k2 * delta / 2.0) / self.mass;
        let k4 = (self.force_accumulator + k3 * delta) / self.mass;
        let acceleration = (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

        self.velocity += acceleration * delta;
        self.position += self.velocity * delta;
    }

    pub fn clear_force_accumulator(&mut self) {
        self.force_accumulator = Vec2::ZERO;
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
        .unwrap()
    }
}
