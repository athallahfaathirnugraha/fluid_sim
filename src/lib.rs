pub mod particle;
pub use particle::*;

pub mod vec2;
pub use vec2::*;

#[derive(Debug)]
pub struct Simulation {
    pub particle_mass: f32,
}
