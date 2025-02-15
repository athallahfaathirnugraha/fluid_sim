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
    /// One cell is `interaction_radius * 2.` by `interaction_radius * 2.`.
    /// Cell `(0, 0)` starts from `(0., 0.)` to `(interaction_radius * 2., interaction_radius * 2.)`.
    cells: HashMap<(i32, i32), Vec<usize>>,
}

impl Simulation {
    /// Must be called after building the simulation with `SimulationBuilder`.
    pub fn init(&mut self) {
        for i in 0..self.particles.len() {
            // update cells
            let particle = self.particles[i];
            let cell = self.get_cell_key(particle.pos);
            self.add_to_cell(i, cell);
        }

        // set prev_pos to equal current pos
        for particle in &mut self.particles {
            particle.prev_pos = particle.pos;
        }
    }
    
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
        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];
            
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

            // update cells
            let particle = self.particles[i];
            
            let prev_cell = self.get_cell_key(particle.prev_pos);
            let curr_cell = self.get_cell_key(particle.pos);

            if prev_cell != curr_cell {
                if self.cells.contains_key(&prev_cell) { self.remove_from_cell(i, prev_cell); }
                self.add_to_cell(i, curr_cell);
            }

            // update pos & prev_pos
            let particle = &mut self.particles[i];

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

        for (cell, indices) in self.cells.iter() {
            let neighbors = self.neighbors_from_cell(*cell);
            
            for &i in indices {
                let mut density = 0.;
                let mut near_density = 0.;

                // compute density
                for &j in &neighbors {
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

                for &j in &neighbors {
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
    }

    fn neighbors_from_cell(&self, cell: (i32, i32)) -> Vec<usize> {
        let cells = [
            (cell.0 - 1, cell.1 - 1),
            (cell.0 + 0, cell.1 - 1),
            (cell.0 + 1, cell.1 - 1),
            (cell.0 - 1, cell.1 + 0),
            (cell.0 + 0, cell.1 + 0),
            (cell.0 + 1, cell.1 + 0),
            (cell.0 - 1, cell.1 + 1),
            (cell.0 + 0, cell.1 + 1),
            (cell.0 + 1, cell.1 + 1),
        ];

        let mut res = vec![];
        for cell in cells {
            if let Some(indices) = self.cells.get(&cell) {
                res.append(&mut indices.clone());
            }
        }

        res
    }

    fn get_cell_key(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / (self.interaction_radius * 2.)).floor() as i32,
            (position.y / (self.interaction_radius * 2.)).floor() as i32,
        )
    }

    // make sure not to add the same particle multiple times
    fn add_to_cell(&mut self, index: usize, cell_key: (i32, i32)) {
        match self.cells.get_mut(&cell_key) {
            Some(cell) => {
                self.particles[index].cell_index = cell.len();
                cell.push(index);
            }
            None => {
                self.particles[index].cell_index = 0;
                self.cells.insert(cell_key, vec![index]);
            }
        }
    }

    fn remove_from_cell(&mut self, index: usize, cell_key: (i32, i32)) {
        let cell = self.cells.get_mut(&cell_key).expect("empty cell");
        self.particles[cell[cell.len() - 1]].cell_index = self.particles[index].cell_index;
        cell.swap_remove(self.particles[index].cell_index);
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
