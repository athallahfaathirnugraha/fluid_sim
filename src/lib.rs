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

    pub fn step(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.vel.y += self.gravity * dt;
        }

        for particle in &mut self.particles {
            particle.prev_pos = particle.pos;
            particle.pos += particle.vel * dt;
        }

        for particle in &mut self.particles {
            particle.vel = (particle.pos - particle.prev_pos) / dt;
        }
    }
}

impl Default for Simulation {
    fn default() -> Simulation {
        Simulation {
            particle_mass: 1.,
            gravity: 196.,
            particles: vec![],
        }
    }
}
