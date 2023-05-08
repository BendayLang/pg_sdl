use crate::Particle;
use pg_sdl::color::Colors;
use pg_sdl::vector2::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Constrain {
    fn draw(&self, canvas: &mut Canvas<Window>, particles: &Vec<Particle>);
}

/// A rod is a constraint.
///
/// It maintains 2 particles at a fixed distance (end1 and end2).
///
/// Its length is determined by the particles' positions.
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

/// A ground is a constraint.
///
/// It maintains a particle at a fixed position.
pub struct Ground {
    particle: usize,
    position: Vec2,
    color: Color,
}
impl Ground {
    pub fn new(particle: usize, color: Color, particles: &Vec<Particle>) -> Self {
        Self {
            particle,
            position: particles[particle].get_position(),
            color,
        }
    }
}
