#![allow(dead_code)]

use pg_sdl::color::{darker, Colors};
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const GRAVITY: Vec2 = Vec2::new_y(9.81);

pub fn apply_gravity(masses: &mut Vec<Mass>) {
    masses.iter_mut().for_each(|mass| {
        mass.apply_force(GRAVITY * mass.mass);
    });
}

/// A mass is a point in space with a velocity and a mass.
pub struct Mass {
    pub position: Vec2,
    pub velocity: Vec2,
    mass: f32,
    radius: f32,
    color: Color,
    force_accumulator: Vec2,
    fixed: bool,
}
impl Mass {
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

pub struct Rod {
    end1: usize,
    end2: usize,
    pub length: f32,
    diameter: f32,
    color: Color,
}
impl Rod {
    pub fn new(end1: usize, end2: usize, length: f32, diameter: f32, color: Color) -> Self {
        Self {
            end1,
            end2,
            length,
            diameter,
            color,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, masses: &Vec<Mass>) {
        let start_position = masses[self.end1].position;
        let end_position = masses[self.end2].position;
        let x_dir = end_position - start_position;
        let y_dir = x_dir.perpendicular() * self.diameter / 2.0;
        let start1 = start_position + y_dir;
        let start2 = start_position - y_dir;
        let end1 = end_position + y_dir;
        let end2 = end_position - y_dir;
        DrawRenderer::thick_line(
            canvas,
            start_position.x as i16,
            start_position.y as i16,
            end_position.x as i16,
            end_position.y as i16,
            self.diameter as u8,
            self.color,
        )
        .unwrap();
        DrawRenderer::line(
            canvas,
            start1.x as i16,
            start1.y as i16,
            end1.x as i16,
            end1.y as i16,
            Colors::BLACK,
        )
        .unwrap();
        DrawRenderer::line(
            canvas,
            start2.x as i16,
            start2.y as i16,
            end2.x as i16,
            end2.y as i16,
            Colors::BLACK,
        )
        .unwrap();
    }
}

/// A spring is a force generator.
///
/// It attracts or repels 2 masses in order to maintain them at a fixed distance.
///
/// The quantities of the spring are:
/// - the **default length** at which the spring is at rest
/// - the force constant **k (N/m)** which is proportional to the deformation of the spring
/// - the damping **b (kg/s)** which is proportional to the velocity of the spring
pub struct Spring {
    end1: usize,
    end2: usize,
    k: f32,
    b: f32,
    default_length: f32,
    diameter: f32,
    color: Color,
}
impl Spring {
    pub fn new(
        start: usize,
        end: usize,
        k: f32,
        b: f32,
        default_length: f32,
        diameter: f32,
        color: Color,
    ) -> Self {
        Self {
            default_length,
            end1: start,
            end2: end,
            k,
            b,
            diameter,
            color,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, masses: &Vec<Mass>) {
        if self.end1 == self.end2 {
            return;
        }

        let start_position = masses[self.end1].position;
        let end_position = masses[self.end2].position;
        let x_dir = end_position - start_position - self.diameter;
        let y_dir = x_dir.perpendicular() * self.diameter;
        let transform = |v: Vec2| {
            start_position - y_dir / 2.0
                + x_dir.with_length(self.diameter / 2.0)
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
            let q = p.perpendicular() * width as f32 / 2.0;
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
            let q = p.perpendicular() * width as f32 / 2.0;
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
            DrawRenderer::line(
                canvas,
                (start.x - q.x) as i16,
                (start.y - q.y) as i16,
                (start.x + q.x) as i16,
                (start.y + q.y) as i16,
                Colors::BLACK,
            )
            .unwrap();
            DrawRenderer::line(
                canvas,
                (end.x - q.x) as i16,
                (end.y - q.y) as i16,
                (end.x + q.x) as i16,
                (end.y + q.y) as i16,
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

        let length = (masses[self.end2].position - masses[self.end1].position).length();
        if length > self.diameter {
            draw_circle(
                start_position + y_dir.with_length(0.0),
                (self.diameter / 4.0) as i16,
                darker(self.color, 0.8),
            );
            draw_thick_line(
                start_position,
                transform(Vec2::new(0.0, 0.5)),
                (self.diameter / 2.0) as u8,
                darker(self.color, 0.8),
            );

            draw_circle(
                end_position,
                (self.diameter / 4.0) as i16,
                darker(self.color, 0.8),
            );
            draw_thick_line(
                transform(Vec2::new(1.0, 0.5)),
                end_position,
                (self.diameter / 2.0) as u8,
                darker(self.color, 0.8),
            );

            let n = 4;
            let dp = 1.0 / n as f32;

            (0..n).for_each(|i| {
                let p = i as f32 / n as f32;
                draw_thick_line(
                    transform(Vec2::new(p, 0.0)),
                    transform(Vec2::new(p + dp / 2.0, 1.0)),
                    (self.diameter / 4.0) as u8,
                    darker(self.color, 0.85),
                );
            });
            (0..n).for_each(|i| {
                let p = i as f32 / n as f32;
                let start = transform(Vec2::new(p + dp / 2.0, 1.0));
                let end = transform(Vec2::new(p + dp, 0.0));
                draw_circle(
                    start,
                    (self.diameter / 3.5) as u8 as i16 / 2 + 1,
                    self.color,
                );
                draw_circle(end, (self.diameter / 3.5) as u8 as i16 / 2 + 1, self.color);
                draw_thick_line(start, end, (self.diameter / 3.5) as u8, self.color);
            });

            draw_rect(
                transform(Vec2::new(0.0, -0.15)),
                transform(Vec2::new(0.0, 1.15)),
                (self.diameter / 3.0) as u8,
                self.color,
            );
            draw_rect(
                transform(Vec2::new(1.0, -0.15)),
                transform(Vec2::new(1.0, 1.15)),
                (self.diameter / 3.0) as u8,
                self.color,
            );
        } else {
            draw_circle(
                start_position,
                (self.diameter / 4.0) as i16,
                darker(self.color, 0.8),
            );
            draw_circle(
                end_position,
                (self.diameter / 4.0) as i16,
                darker(self.color, 0.8),
            );
            draw_thick_line(
                start_position,
                end_position,
                (self.diameter / 2.0) as u8,
                darker(self.color, 0.8),
            );

            let delta = end_position - start_position;
            draw_rect(
                start_position + delta / 2.0 + y_dir.normalized() * self.diameter * 1.3 / 2.0,
                start_position + delta / 2.0 - y_dir.normalized() * self.diameter * 1.3 / 2.0,
                (self.diameter / 3.0) as u8,
                self.color,
            );
        }

        draw_circle(start_position, (self.diameter / 6.0) as i16, self.color);
        draw_circle(end_position, (self.diameter / 6.0) as i16, self.color);
    }

    pub fn apply_force(&mut self, masses: &mut Vec<Mass>) {
        if self.end1 == self.end2 {
            return;
        }

        let delta_position = masses[self.end2].position - masses[self.end1].position;
        let delta_velocity = masses[self.end2].velocity - masses[self.end1].velocity;
        let direction = delta_position.normalized();

        let force = -self.k * (delta_position.length() - self.default_length);
        let damping = -self.b * delta_velocity.dot(direction);
        let force = direction * (force + damping);

        masses[self.end1].apply_force(-force);
        masses[self.end2].apply_force(force);
    }

    pub fn change_end(&mut self, new_end: usize) {
        self.end2 = new_end;
    }
}

/// A motor moves a mass around another mass in a circle with a constant speed.
pub struct Motor {
    start: usize,
    end: usize,
    speed: f32,
    color: Color,
}

impl Motor {
    pub fn new(start: usize, end: usize, speed: f32, color: Color) -> Self {
        Self {
            start,
            end,
            speed,
            color,
        }
    }

    pub fn update(&self, delta: f32, masses: &mut Vec<Mass>) {
        let start_position = masses[self.start].position;
        let end_position = masses[self.end].position;
        let delta_position = end_position - start_position;

        masses[self.end].position = start_position + delta_position.rotated(self.speed * delta);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, masses: &Vec<Mass>) {
        let start_position = masses[self.start].position;
        let end_position = masses[self.end].position;
        let delta_position = end_position - start_position;
        let radius = delta_position.length();
        let angle = delta_position.angle_deg();

        let w = 5;

        DrawRenderer::filled_circle(
            canvas,
            start_position.x as i16,
            start_position.y as i16,
            radius as i16 + w,
            self.color,
        )
        .unwrap();

        DrawRenderer::circle(
            canvas,
            start_position.x as i16,
            start_position.y as i16,
            radius as i16 - w,
            Colors::BLACK,
        )
        .unwrap();

        DrawRenderer::circle(
            canvas,
            start_position.x as i16,
            start_position.y as i16,
            radius as i16 + w,
            Colors::BLACK,
        )
        .unwrap();

        (0..4).into_iter().for_each(|i| {
            let angle = angle + i as f32 / 4.0 * 360.0;
            let t1 = start_position + Vec2::from_polar_deg(radius + w as f32 - 1.0, angle);
            let t2 = start_position + Vec2::from_polar_deg(radius + w as f32 - 2.0, angle + 45.0);
            DrawRenderer::thick_line(
                canvas,
                start_position.x as i16,
                start_position.y as i16,
                t1.x as i16,
                t1.y as i16,
                2 * w as u8,
                Colors::BLACK,
            )
            .unwrap();
            DrawRenderer::filled_pie(
                canvas,
                t1.x as i16,
                t1.y as i16,
                w,
                angle as i16 - 90,
                angle as i16 + 90,
                Colors::BLACK,
            )
            .unwrap();
            DrawRenderer::filled_pie(
                canvas,
                t2.x as i16,
                t2.y as i16,
                w * 2,
                angle as i16 - 45,
                angle as i16 + 135,
                self.color,
            )
            .unwrap();
            DrawRenderer::pie(
                canvas,
                t2.x as i16,
                t2.y as i16,
                w * 2,
                angle as i16 - 45,
                angle as i16 + 135,
                Colors::BLACK,
            )
            .unwrap();
        });
    }
}

fn main() {}
