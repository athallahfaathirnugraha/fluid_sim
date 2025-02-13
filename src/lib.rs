pub mod particle;
pub use particle::*;

pub mod vec2;
pub use vec2::*;

pub mod rect;
pub use rect::*;

pub mod simulation_builder;
pub use simulation_builder::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Simulation {
    pub gravity: f32,
    pub boundaries: Rect,
    pub interaction_radius: f32,
    pub pressure_multiplier: f32,
    pub near_pressure_multiplier: f32,
    pub rest_density: f32,
    particles: Vec<Particle>,
    cells: HashMap<(i32, i32), Vec<usize>>,
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

    fn add_to_cell(&mut self, index: usize, cell_key: (i32, i32)) {
        match self.cells.get_mut(&cell_key) {
            Some(cell) => { cell.push(index); }
            None => { self.cells.insert(cell_key, vec![index]); }
        }
    }

    fn remove_from_cell(&mut self, index: usize, cell_key: (i32, i32)) {
        let cell = self.cells.get_mut(&cell_key).expect("empty cell");

        for i in 0..cell.len() {
            if cell[i] == index {
                cell.swap_remove(i);
                return;
            }
        }
    }
}

impl Default for Simulation {
    fn default() -> Simulation {
        SimulationBuilder::default().build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_to_cell() {
        let mut simulation = Simulation::default();
        simulation.add_to_cell(0, (0, 0));
        assert_eq!(*simulation.cells.get(&(0, 0)).unwrap(), vec![0]);
        simulation.add_to_cell(1, (0, 0));
        assert_eq!(*simulation.cells.get(&(0, 0)).unwrap(), vec![0, 1]);
    }

    #[test]
    fn remove_from_cell() {
        let mut simulation = Simulation::default();
        simulation.cells.insert((0, 0), vec![0, 1, 2, 3]);
        simulation.remove_from_cell(1, (0, 0));
        assert_eq!(*simulation.cells.get(&(0, 0)).unwrap(), vec![0, 3, 2]);
        simulation.remove_from_cell(3, (0, 0));
        assert_eq!(*simulation.cells.get(&(0, 0)).unwrap(), vec![0, 2]);
    }
}
