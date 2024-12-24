pub mod world;
pub mod particle;

pub use world::*;
pub use particle::*;

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
