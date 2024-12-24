use crate::*;

#[derive(Debug, Clone)]
pub struct World {
    particles: Vec<Particle>,
    boundaries: Vec2,
    coll_damping: f32,
}

impl World {
    pub fn new(particles: Vec<Particle>, boundaries: Vec2, coll_damping: f32) -> World {
        World { particles, boundaries, coll_damping }
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }
    
    /// Assumes that `dt` will stay constant.
    pub fn step(&mut self, dt: f32) {
        let gravity = 60. * 14.;
        
        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];

            particle.calculate_forces(gravity, dt);
            particle.update_vel(dt);
            particle.update_pos(dt);

            if particle.pos.y < 0. {
                particle.pos.y = 0.;
                particle.vel.y *= -self.coll_damping;
            }

            if particle.pos.x < 0. {
                particle.pos.x = 0.;
                particle.vel.x *= -self.coll_damping;
            }

            if particle.pos.y > self.boundaries.y {
                particle.pos.y = self.boundaries.y;
                particle.vel.y *= -self.coll_damping;
            }

            if particle.pos.x > self.boundaries.x {
                particle.pos.x = self.boundaries.x;
                particle.vel.x *= -self.coll_damping;
            }

            particle.reset_forces();
        }
    }
}
