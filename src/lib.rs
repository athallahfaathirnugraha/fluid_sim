pub mod world;

pub use world::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn dist(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Vec2 {
        let dist = self.dist();
        Vec2 { x: self.x / dist, y: self.y / dist }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle {
        Particle {
            pos: Vec2 { x, y },
            vel: Vec2 { x: 0., y: 0. },
        }
    }
}
