pub mod particle;
pub use particle::*;

pub mod vec2;
pub use vec2::*;

#[derive(Debug)]
pub struct Simulation {
    pub particle_mass: f32,
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }
}

impl Default for Simulation {
    fn default() -> Simulation {
        Simulation {
            particle_mass: 1.,
            particles: vec![],
        }
    }
}
