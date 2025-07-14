extern crate nalgebra_glm as glm;

pub struct Panning {
    pub mode: bool,
    pub last_pos: glm::Vec2,
    pub speed: f32
}