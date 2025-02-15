use crate::*;

// TODO: probably should create a constructor so user doesnt have to specify cache fields
// like prev_pos & cell_index
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub prev_pos: Vec2,
    /// The particle's index in its current cell.
    pub cell_index: usize,
}
