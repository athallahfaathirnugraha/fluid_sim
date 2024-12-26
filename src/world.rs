use crate::*;

#[derive(Debug, Clone)]
pub struct World {
    pub particles: Vec<Particle>,
    pub boundaries: Vec2,
    pub coll_damping: f32,
    pub particle_mass: f32,
}

impl World {
    pub fn with_particles(particles: Vec<Particle>) -> World {
        World {
            particles,
            ..Default::default()
        }
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }
    
    /// Assumes that `dt` will stay constant.
    pub fn step(&mut self, dt: f32) {
        let gravity = 60. * 14.;
        
        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];

            particle.calculate_forces(gravity, self.particle_mass);
            particle.update_vel(self.particle_mass, dt);
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

impl Default for World {
    fn default() -> World {
        World {
            particles: vec![],
            boundaries: Vec2 { x: 400., y: 400. },
            coll_damping: 0.5,
            particle_mass: 1.,
        }
    }
}
