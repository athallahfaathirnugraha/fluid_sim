use crate::*;

#[derive(Debug, Clone)]
pub struct World {
    particles: Vec<Particle>,
    boundaries: Vec2,
}

impl World {
    pub fn new(particles: Vec<Particle>, boundaries: Vec2) -> World {
        World { particles, boundaries }
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }
    
    /// Assumes that `dt` will stay constant.
    pub fn step(&mut self, dt: f32) {
        let gravity = 60. * 14.;
        
        for particle in &mut self.particles {
            particle.vel.y += gravity * dt;

            particle.pos.x += particle.vel.x * dt;
            particle.pos.y += particle.vel.y * dt;

            let mut reverse_y_vel = || {
                particle.vel.y *= -1.;
                // to prevent the particle from bouncing lower each time
                particle.vel.y -= gravity * dt;
            };

            if particle.pos.y < 0. {
                particle.pos.y = 0.;
                reverse_y_vel();
            }

            if particle.pos.x < 0. {
                particle.pos.x = 0.;
                particle.vel.x *= -1.;
            }

            if particle.pos.y > self.boundaries.y {
                particle.pos.y = self.boundaries.y;
                reverse_y_vel();
            }

            if particle.pos.x > self.boundaries.x {
                particle.pos.x = self.boundaries.x;
                particle.vel.x *= -1.;
            }
        }
    }
}
