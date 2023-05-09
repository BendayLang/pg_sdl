use crate::Particle;
use ndarray::Array2;
use pg_sdl::color::Colors;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Constrain {
    fn constrain_matrix(&self, particles: &Vec<Particle>) -> Array2<f32>;
    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>);
}

/// A rod is a constraint.
///
/// It maintains 2 particles at a fixed distance (end1 and end2).
///
/// Its length is determined by the particles' positions at the time of creation.
pub struct Rod {
    end1: usize,
    end2: usize,
    length: f32,
    diameter: f32,
    color: Color,
}
impl Rod {
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

impl Constrain for Rod {
    fn constrain_matrix(&self, particles: &Vec<Particle>) -> Array2<f32> {
        let mut matrix = Array2::<f32>::zeros((2, particles.len() * 2));
        let start_position = particles[self.end1].get_position();
        let end_position = particles[self.end2].get_position();
        matrix
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
pub struct Fixed {
    particle: usize,
    position: Vec2,
    color: Color,
}
impl Fixed {
    pub fn new(particle: usize, color: Color, particles: &Vec<Particle>) -> Self {
        Self {
            particle,
            position: particles[particle].get_position(),
            color,
        }
    }
}
impl Constrain for Fixed {
    fn constrain_matrix(&self, particles: &Vec<Particle>) -> Array2<f32> {
        todo!()
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
pub struct Line {
    particle: usize,
    point: Vec2,
    director: Vec2,
    color: Color,
}
impl Line {
    pub fn new(particle: usize, director: Vec2, color: Color, particles: &Vec<Particle>) -> Self {
        Self {
            particle,
            point: particles[particle].get_position(),
            director,
            color,
        }
    }
}
