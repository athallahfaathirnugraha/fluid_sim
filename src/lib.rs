pub mod particle;
pub use particle::*;

pub mod vec2;
pub use vec2::*;

pub mod rect;
pub use rect::*;

#[derive(Debug, Clone)]
pub struct SimulationBuilder {
    pub gravity: f32,
    pub boundaries: Rect,
    pub interaction_radius: f32,
    pub pressure_multiplier: f32,
    pub near_pressure_multiplier: f32,
    pub rest_density: f32,
    pub particles: Vec<Particle>,
}

impl SimulationBuilder {
    pub fn with_gravity(&self, gravity: f32) -> SimulationBuilder {
        SimulationBuilder {
            gravity,
            ..self.clone()
        }
    }

    pub fn with_boundaries(&self, boundaries: Rect) -> SimulationBuilder {
        SimulationBuilder {
            boundaries,
            ..self.clone()
        }
    }

    pub fn with_interaction_radius(&self, interaction_radius: f32) -> SimulationBuilder {
        SimulationBuilder {
            interaction_radius,
            ..self.clone()
        }
    }

    pub fn with_pressure_multiplier(&self, pressure_multiplier: f32) -> SimulationBuilder {
        SimulationBuilder {
            pressure_multiplier,
            ..self.clone()
        }
    }

    pub fn with_near_pressure_multiplier(&self, near_pressure_multiplier: f32) -> SimulationBuilder {
        SimulationBuilder {
            near_pressure_multiplier,
            ..self.clone()
        }
    }

    pub fn rest_density(&self, rest_density: f32) -> SimulationBuilder {
        SimulationBuilder {
            rest_density,
            ..self.clone()
        }
    }

    pub fn with_particles(&self, particles: Vec<Particle>) -> SimulationBuilder {
        SimulationBuilder {
            particles,
            ..self.clone()
        }
    }

    pub fn build(&self) -> Simulation {
        Simulation {
            gravity: self.gravity,
            boundaries: self.boundaries,
            interaction_radius: self.interaction_radius,
            pressure_multiplier: self.pressure_multiplier,
            near_pressure_multiplier: self.near_pressure_multiplier,
            rest_density: self.rest_density,
            particles: self.particles.clone(),
        }
    }
}

impl Default for SimulationBuilder {
    fn default() -> SimulationBuilder {
        SimulationBuilder {
            gravity: 196.,
            boundaries: Rect {
                min: Vec2 { x: 0., y: 0. },
                max: Vec2 { x: 0., y: 0. },
            },
            interaction_radius: 40.,
            pressure_multiplier: 45.,
            near_pressure_multiplier: 45.,
            rest_density: 9.,
            particles: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Simulation {
    pub gravity: f32,
    pub boundaries: Rect,
    pub interaction_radius: f32,
    pub pressure_multiplier: f32,
    pub near_pressure_multiplier: f32,
    pub rest_density: f32,
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
                particle.vel.x *= -0.5;
            }
            
            if particle.pos.x > self.boundaries.max.x {
                particle.pos.x = self.boundaries.max.x;
                particle.vel.x *= -0.5;
            }
            
            if particle.pos.y < self.boundaries.min.y {
                particle.pos.y = self.boundaries.min.y;
                particle.vel.y *= -0.5;
            }
            
            if particle.pos.y > self.boundaries.max.y {
                particle.pos.y = self.boundaries.max.y;
                particle.vel.y *= -0.5;
            }
            
            particle.vel.y += self.gravity * dt;
        }

        for particle in &mut self.particles {
            particle.prev_pos = particle.pos;
            particle.pos += particle.vel * dt;
        }

        self.density_relaxation(dt);

        for particle in &mut self.particles {
            particle.vel = (particle.pos - particle.prev_pos) / dt;
        }
    }

    fn density_relaxation(&mut self, dt: f32) {
        let interaction_radius = self.interaction_radius;
        let pressure_multiplier = self.pressure_multiplier;
        let near_pressure_multiplier = self.near_pressure_multiplier;
        let rest_density = self.rest_density;
        
        for i in 0..self.particles.len() {
            let mut density = 0.;
            let mut near_density = 0.;

            // TODO: only neighbors
            // compute density
            for j in 0..self.particles.len() {
                if i == j { continue; }

                let particle_i = self.particles[i];
                let particle_j = self.particles[j];

                let dist = Vec2::dist(particle_i.pos - particle_j.pos);
                let q = dist / interaction_radius;

                if q < 1. {
                    density += (1. - q) * (1. - q);
                    near_density += (1. - q) * (1. - q) * (1. - q);
                }
            }

            // compute pressure
            let pressure = pressure_multiplier * (density - rest_density);
            let near_pressure = near_pressure_multiplier * near_density;

            let mut dpos = Vec2 { x: 0., y: 0. };

            // TODO: only neighbors
            for j in 0..self.particles.len() {
                if i == j { continue; }

                let particle_i = self.particles[i];
                let particle_j = &mut self.particles[j];

                let diff = particle_j.pos - particle_i.pos;
                let dist = Vec2::dist(diff);
                let q = dist / interaction_radius;

                if q < 1. {
                    let displacement = diff.normalize() * (pressure * (1. - q) + near_pressure * (1. - q) * (1. - q)) * dt * dt;
                    particle_j.pos += displacement / 2.;
                    dpos -= displacement / 2.;
                }
            }

            self.particles[i].pos += dpos;
        }
    }

    fn neighbors(&self, particle_index: usize) -> Vec<usize> {
        todo!()
    }
}

impl Default for Simulation {
    fn default() -> Simulation {
        SimulationBuilder::default().build()
    }
}
