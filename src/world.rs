use crate::*;

#[derive(Debug, Clone)]
pub struct World {
    particles: Vec<Particle>,
    pub boundaries: Vec2,
    pub coll_damping: f32,
    pub particle_mass: f32,
    pub density_radius: f32,
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

    pub fn update_densities(&mut self) {
        for i in 0..self.particles.len() {
            let mut particle = self.particles[i];
            particle.density = particle.density(self.density_radius, &self.particles, self.particle_mass);
            self.particles[i] = particle;
        }
    }
    
    /// Assumes that `dt` will stay constant.
    pub fn step(&mut self, dt: f32) {
        let gravity = 60. * 14.;

        self.update_densities();
        
        for i in 0..self.particles.len() {
            // copy the particle from vec
            let mut particle = self.particles[i];

            particle.reset_forces();
            particle.calculate_forces(gravity, self.particle_mass, &self.particles);
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

            // copy the updated particle to the original vec
            self.particles[i] = particle;
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
            density_radius: 10.,
        }
    }
}
