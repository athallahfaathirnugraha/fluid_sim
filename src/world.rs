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
            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }

                let particle = self.particles[i];
                let other_particle = self.particles[j];

                let diff = Vec2 {
                    x: particle.pos.x - other_particle.pos.x,
                    y: particle.pos.y - other_particle.pos.y,
                };

                let dist = diff.dist();

                let mut pressure_force = diff.normalize();

                pressure_force.x *= f32::exp(-dist * dist) * 10000.;
                pressure_force.y *= f32::exp(-dist * dist) * 10000.;

                if pressure_force.x.is_nan() || pressure_force.y.is_nan() {
                    pressure_force = Vec2 { x: 10., y: 0. };
                }

                self.particles[i].forces.x += pressure_force.x;
                self.particles[i].forces.y += pressure_force.y;

                self.particles[j].forces.x -= pressure_force.x;
                self.particles[j].forces.y -= pressure_force.y;
            }
        }
        
        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];

            particle.forces.y += gravity;

            particle.vel.x += particle.forces.x * dt;
            particle.vel.y += particle.forces.y * dt;

            particle.pos.x += particle.vel.x * dt;
            particle.pos.y += particle.vel.y * dt;

            let mut reverse_y_vel = || {
                particle.vel.y *= -self.coll_damping;
                // to prevent the particle from bouncing lower each time
                particle.vel.y -= gravity * dt;
            };

            if particle.pos.y < 0. {
                particle.pos.y = 0.;
                reverse_y_vel();
            }

            if particle.pos.x < 0. {
                particle.pos.x = 0.;
                particle.vel.x *= -self.coll_damping;
            }

            if particle.pos.y > self.boundaries.y {
                particle.pos.y = self.boundaries.y;
                reverse_y_vel();
            }

            if particle.pos.x > self.boundaries.x {
                particle.pos.x = self.boundaries.x;
                particle.vel.x *= -self.coll_damping;
            }

            // reset forces
            particle.forces = Vec2 { x: 0., y: 0. };
        }
    }
}
