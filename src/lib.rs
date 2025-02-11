pub mod particle;
pub use particle::*;

pub mod vec2;
pub use vec2::*;

pub mod rect;
pub use rect::*;

#[derive(Debug)]
pub struct Simulation {
    pub particle_mass: f32,
    pub gravity: f32,
    pub boundaries: Rect,
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
            if particle.pos.x < self.boundaries.min.x {
                particle.pos.x = self.boundaries.min.x;
                particle.vel.x *= -1.;
            }
            
            if particle.pos.x > self.boundaries.max.x {
                particle.pos.x = self.boundaries.max.x;
                particle.vel.x *= -1.;
            }
            
            if particle.pos.y < self.boundaries.min.y {
                particle.pos.y = self.boundaries.min.y;
                particle.vel.y *= -1.;
            }
            
            if particle.pos.y > self.boundaries.max.y {
                particle.pos.y = self.boundaries.max.y;
                particle.vel.y *= -1.;
            }
            
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
            boundaries: Rect {
                min: Vec2 { x: 0., y: 0. },
                max: Vec2 { x: 0., y: 0. }
            },
            particles: vec![],
        }
    }
}
