use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub prev_pos: Vec2,
}
