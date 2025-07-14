extern crate nalgebra_glm as glm;

use crate::entity::Transform::Transform;
use crate::graphics::texture2d::Texture2D;

pub trait Renderer {
    fn draw(&mut self, texture: &Texture2D, transform: Transform);
}