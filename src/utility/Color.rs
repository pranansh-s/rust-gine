extern crate nalgebra_glm as glm;

#[derive(Debug)]
#[derive(Clone)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn into_glm_vec4(&self) -> glm::Vec4 {
        glm::vec4(self.0, self.1, self.2, self.3)
    }
}