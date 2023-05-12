use crate::Particle;
use pg_sdl::color::Colors;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Constrain {
    /// The constraint function is such that it returns 0 when the constraint is respected
    fn constrain_function(&self, particles: &Vec<Particle>) -> f32;
    /// The derivative of the constraint function (with respect to time)
    fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32;
    /// The jacobian blocs of the constraint function (with respect to the particles)
    fn jacobian_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)>;
    /// The jacobian blocs of the constraint derivative function (with respect to the particles)
    fn jacobian_derivative_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)>;
    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>);
}

/// A rod is a constraint.
///
/// It maintains 2 particles at a fixed distance (end1 and end2).
///
/// Its length is determined by the particles' positions at the time of creation.
pub struct LengthConstraint {
    end1: usize,
    end2: usize,
    length: f32,
    diameter: f32,
    color: Color,
}
impl LengthConstraint {
    pub fn new(
        end1: usize,
        end2: usize,
        diameter: f32,
        color: Color,
        particles: &Vec<Particle>,
    ) -> Self {
        Self {
            end1,
            end2,
            length: (particles[end1].get_position() - particles[end2].get_position()).length(),
            diameter,
            color,
        }
    }
}

impl Constrain for LengthConstraint {
    fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
        let position1 = particles[self.end1].get_position();
        let position2 = particles[self.end2].get_position();
        (position2 - position1).length_squared() - self.length.powf(2.0)
    }
    fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
        let velocity1 = particles[self.end1].get_velocity();
        let velocity2 = particles[self.end2].get_velocity();
        (velocity2 - velocity1).length_squared()
    }
    fn jacobian_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)> {
        let position1 = particles[self.end1].get_position();
        let position2 = particles[self.end2].get_position();
        Vec::from([
            (
                self.end1 * 2,
                2.0 * (position1.x - position2.x)
                    + position2.x.powf(2.0)
                    + (position2.y - position1.y).powf(2.0),
            ),
            (
                self.end1 * 2 + 1,
                2.0 * (position1.y - position2.y)
                    + position2.y.powf(2.0)
                    + (position2.x - position1.x).powf(2.0),
            ),
            (
                self.end2 * 2,
                2.0 * (position2.x - position1.x)
                    + position1.x.powf(2.0)
                    + (position1.y - position2.y).powf(2.0),
            ),
            (
                self.end2 * 2 + 1,
                2.0 * (position2.y - position1.y)
                    + position1.y.powf(2.0)
                    + (position1.x - position2.x).powf(2.0),
            ),
        ])
    }
    fn jacobian_derivative_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
        Vec::from([
            (self.end1 * 2, 1.0),
            (self.end1 * 2 + 1, 1.0),
            (self.end2 * 2, -1.0),
            (self.end2 * 2 + 1, -1.0),
        ])
    }

    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {
        let start_position = particles[self.end1].get_position();
        let end_position = particles[self.end2].get_position();
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

/// A fixed is a constraint.
///
/// It maintains a particle at a fixed position.
pub struct FixedConstraint {
    particle: usize,
    position: Vec2,
    color: Color,
}
impl FixedConstraint {
    pub fn new(particle: usize, color: Color, particles: &Vec<Particle>) -> Self {
        Self {
            particle,
            position: particles[particle].get_position(),
            color,
        }
    }
}
impl Constrain for FixedConstraint {
    fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
        let position = particles[self.particle].get_position();
        let delta = position - self.position;
        delta.length()
    }

    fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
        let velocity = particles[self.particle].get_velocity();
        velocity.length()
    }

    fn jacobian_blocs(&self, particles: &Vec<Particle>) -> Vec<(usize, f32)> {
        let position = particles[self.particle].get_position();
        let delta = position - self.position;
        Vec::from([(self.particle * 2, 1.0), (self.particle * 2 + 1, 1.0)])
    }

    fn jacobian_derivative_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
        Vec::from([(self.particle * 2, 0.0), (self.particle * 2 + 1, 0.0)])
    }

    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {
        let position = particles[self.particle].get_position();
        DrawRenderer::line(
            canvas,
            position.x as i16,
            position.y as i16,
            self.position.x as i16,
            self.position.y as i16,
            self.color,
        )
        .unwrap();
    }
}

/// A line is a constraint.
///
/// It maintains a particle on a line defined by a point and a director vector.
pub struct LineConstraint {
    particle: usize,
    point: Vec2,
    director: Vec2,
    color: Color,
}
impl LineConstraint {
    pub fn new(particle: usize, director: Vec2, color: Color, particles: &Vec<Particle>) -> Self {
        Self {
            particle,
            point: particles[particle].get_position(),
            director,
            color,
        }
    }
}
impl Constrain for LineConstraint {
    fn constrain_function(&self, particles: &Vec<Particle>) -> f32 {
        let position = particles[self.particle].get_position();
        (position - self.point).dot(self.director)
    }

    fn constrain_derivative(&self, particles: &Vec<Particle>) -> f32 {
        let velocity = particles[self.particle].get_velocity();
        velocity.dot(self.director)
    }

    fn jacobian_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
        Vec::new()
    }

    fn jacobian_derivative_blocs(&self, _particles: &Vec<Particle>) -> Vec<(usize, f32)> {
        Vec::new()
    }

    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>) {}
}
