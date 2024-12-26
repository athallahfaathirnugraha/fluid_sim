use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub forces: Vec2,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        Particle {
            pos: Vec2 { x, y },
            vel: Vec2 { x: 0., y: 0. },
            forces: Vec2 { x: 0., y: 0. },
        }
    }

    pub fn reset_forces(&mut self) {
        self.forces = Vec2 { x: 0., y: 0. };
    }

    pub fn calculate_forces(&mut self, gravity: f32, particle_mass: f32) {
        self.forces.y += gravity * particle_mass;
    }

    pub fn update_vel(&mut self, particle_mass: f32, dt: f32) {
        self.vel += self.forces / particle_mass * dt;
    }

    pub fn update_pos(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }
}
