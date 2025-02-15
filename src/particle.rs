use crate::*;

// like prev_pos & cell_index
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub prev_pos: Vec2,
    /// The particle's index in its current cell.
    pub cell_index: usize,
}

impl Particle {
    pub fn new(
        pos: Vec2,
        vel: Vec2,
    ) -> Particle {
        Particle {
            pos,
            vel,
            prev_pos: Vec2 { x: 0., y: 0. },
            cell_index: 0,
        }
    }
}
