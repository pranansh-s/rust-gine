extern crate nalgebra_glm as glm;

use crate::entity::renderer::Renderer;
use crate::entity::Transform::Transform;
use crate::graphics::shader_program::ShaderProgram;
use crate::graphics::vertex_array::VertexArray;
use crate::graphics::texture2d::Texture2D;
use crate::graphics::buffer::Buffer;

pub struct SpriteRenderer {
    shader_program: ShaderProgram,
    quad_vao: VertexArray,
}

impl SpriteRenderer {
    pub fn new(shader_program: ShaderProgram) -> Self {
        let vertices: [f32; 24] = [ 
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 
    
            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];

        let vbo: Buffer;
        let quad_vao: VertexArray;
        unsafe {
            vbo = Buffer::new(gl::ARRAY_BUFFER).unwrap();
            quad_vao = VertexArray::new().unwrap();
    
            quad_vao.bind();
            vbo.bind(&vertices, gl::STATIC_DRAW);
            
            quad_vao.set_attrb::<f32>(0, 4, 4, 0);
    
            vbo.unbind();
            quad_vao.unbind();
        }
        
        SpriteRenderer {
            shader_program,
            quad_vao,
        }
    }
}

impl Renderer for SpriteRenderer {
    fn draw(&mut self, texture: &Texture2D, transform: Transform) {
        unsafe { 
            self.shader_program.apply();
            texture.apply();
        }

        let mut model = glm::Mat4::identity();

        model = glm::translate(&model, &glm::vec3(transform.position.x, transform.position.y, 0.0));

        model = glm::translate(&model, &glm::vec3(transform.scale.x / 2.0, transform.scale.y / 2.0, 0.0));
        model = glm::rotate(&model, (transform.rotation).to_radians(), &glm::vec3(0.0, 0.0, 1.0));
        model = glm::translate(&model, &glm::vec3(-transform.scale.x / 2.0, -transform.scale.y / 2.0, 0.0));

        model = glm::scale(&model, &glm::vec3(transform.scale.x, transform.scale.y, 1.0));

        unsafe {
            self.shader_program.set_mat4("model", &model);
            
            self.quad_vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            self.quad_vao.unbind();
        }
    }
}
