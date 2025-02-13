use crate::*;

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
