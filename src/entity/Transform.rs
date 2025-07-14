extern crate nalgebra_glm as glm;

#[derive(Clone)]
pub struct Transform {
    pub position: glm::Vec2,
    pub rotation: f32,
    pub scale: glm::Vec2
}