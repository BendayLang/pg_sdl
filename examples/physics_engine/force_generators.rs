use crate::particle::Particle;
use pg_sdl::color::{darker, Colors};
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait ForceGenerator {
    fn apply_forces(&self, particles: &mut Vec<Particle>);
    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>);
}

pub struct Gravity {
    acceleration: Vec2,
}
impl Gravity {
    pub fn new(acceleration: Vec2) -> Self {
        Self { acceleration }
    }
}
impl ForceGenerator for Gravity {
    fn apply_forces(&self, particles: &mut Vec<Particle>) {
        particles.iter_mut().for_each(|particle| {
            particle.apply_force(self.acceleration * particle.get_mass());
        });
    }
    fn draw(&self, _canvas: &mut Canvas<Window>, _particles: &Vec<Particle>) {}
}

/// A physics_engine is a force generator.
///
/// It attracts or repels 2 particles (end1 and end2), forcing them to maintain a fixed distance.
///
/// The quantities of the physics_engine are:
/// - the **rest length**
/// - the force constant **k** (N/m)
/// - the damping **b** (kg/s)
pub struct Spring {
    end1: usize,
    end2: usize,
    k: f32,
    b: f32,
    rest_length: f32,
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
            rest_length: default_length,
            end1: start,
            end2: end,
            k,
            b,
            diameter,
            color,
        }
    }
}
impl ForceGenerator for Spring {
    fn apply_forces(&self, particles: &mut Vec<Particle>) {
        if self.end1 == self.end2 {
            return;
        }

        let delta_position =
            particles[self.end2].get_position() - particles[self.end1].get_position();
        let delta_velocity =
            particles[self.end2].get_velocity() - particles[self.end1].get_velocity();
        let direction = delta_position.normalized();

        let force = -self.k * (delta_position.length() - self.rest_length);
        let damping = -self.b * delta_velocity.dot(direction);
        let force = direction * (force + damping);

        particles[self.end1].apply_force(-force);
        particles[self.end2].apply_force(force);
    }

    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {
        if self.end1 == self.end2 {
            return;
        }

        let start_position = particles[self.end1].get_position();
        let end_position = particles[self.end2].get_position();
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

        let length =
            (particles[self.end2].get_position() - particles[self.end1].get_position()).length();
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
}

/// A motor is a force generator.
///
/// It applies a force to a particle (start) to make it rotate around another particle (end).
///
/// It is generally used in conjunction with a `Rod` to make a motorized joint.
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
}
impl ForceGenerator for Motor {
    fn apply_forces(&self, particles: &mut Vec<Particle>) {
        let start_position = particles[self.start].get_position();
        let end_position = particles[self.end].get_position();
        let delta_position = end_position - start_position;

        // particles[self.end].position = start_position + delta_position.rotated(self.speed * delta);
        particles[self.end].apply_force(delta_position.perpendicular() * 100.0);
    }

    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {
        let start_position = particles[self.start].get_position();
        let end_position = particles[self.end].get_position();
        let delta_position = end_position - start_position;
        let radius = delta_position.length();
        let angle = delta_position.angle_deg();

        let w = 5;

        if radius as i16 - w <= 0 {
            return;
        }

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
