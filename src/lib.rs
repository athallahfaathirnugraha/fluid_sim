pub mod particle;
pub use particle::*;

pub mod vec2;
pub use vec2::*;

#[derive(Debug)]
pub struct Simulation {
    pub particle_mass: f32,
    pub gravity: f32,
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn with_particles(particles: Vec<Particle>) -> Simulation {
        Simulation {
            particles,
            ..Simulation::default()
        }
    }
    
    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn step(&mut self, dt: f32) {}
}

impl Default for Simulation {
    fn default() -> Simulation {
        Simulation {
            particle_mass: 1.,
            gravity: 9.8,
            particles: vec![],
        }
    }
}
