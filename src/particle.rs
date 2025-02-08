use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub forces: Vec2,
    pub density: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        Particle {
            pos: Vec2 { x, y },
            vel: Vec2 { x: 0., y: 0. },
            forces: Vec2 { x: 0., y: 0. },
            density: 0.,
        }
    }

    pub fn reset_forces(&mut self) {
        self.forces = Vec2 { x: 0., y: 0. };
    }

    pub fn calculate_forces(
        &mut self,
        gravity: f32,
        particle_mass: f32,
        target_density: f32,
        neighbors: &Vec<Particle>
    ) {
        self.forces.y += gravity * particle_mass;

        for neighbor in neighbors {
            let pressure_force = Particle::pressure_force(self, neighbor, target_density);

            let mut pressure_dir = (self.pos - neighbor.pos).normalize();

            if pressure_dir == (Vec2 { x: 0., y: 0. }) {
                use rand::Rng;

                let mut rng = rand::thread_rng();

                pressure_dir = Vec2 {
                    x: rng.gen::<f32>(),
                    y: rng.gen::<f32>(),
                };
            }

            self.forces += pressure_dir * pressure_force;
        }
    }

    pub fn pressure_force(a: &Particle, b: &Particle, target_density: f32) -> f32 {
        let density = f32::max(a.density, b.density);
        let multiplier = f32::max(1., 100. - Vec2::dist(a.pos - b.pos));
        f32::max(0., (density - target_density) * multiplier)
    }

    pub fn update_vel(&mut self, particle_mass: f32, dt: f32) {
        self.vel += self.forces / particle_mass * dt;
    }

    pub fn update_pos(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    /// The current particle should be included in `neighbors`.
    pub fn density(&self, radius: f32, neighbors: &Vec<Particle>, particle_mass: f32) -> f32 {
        let area = std::f32::consts::PI * radius * radius;
        let radius_sqr = radius * radius;

        let mut len = 0;

        for &neighbor in neighbors {
            if (self.pos - neighbor.pos).dist_sqr() <= radius_sqr {
                len += 1;
            }
        }

        len as f32 * particle_mass / area
    }
}
