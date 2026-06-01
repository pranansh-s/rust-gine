extern crate nalgebra_glm as glm;

use crate::graphics::shader_program::ShaderProgram;
use crate::graphics::vertex_array::VertexArray;
use crate::graphics::buffer::Buffer;
use crate::resource_manager::asset_manager::AssetManager;
use crate::ui::ui_component::Div;

pub struct UiRenderer<'c> {
    width: u32,
    height: u32,
    shader_program: ShaderProgram,
    vao: VertexArray,
    node: &'c Div,
}

impl<'c> UiRenderer<'c> {
    pub fn new(shader_name: &'static str, node: &'c Div, width: u32, height: u32, manager: &mut AssetManager) -> Self {
        let vertex_shader = format!("shaders/{}/vertex.vs", shader_name);
        let fragment_shader = format!("shaders/{}/fragment.fs", shader_name);  

        manager.load_shader_program(&vertex_shader, &fragment_shader, None, shader_name);
        let shader_program = manager.get_shader_program(shader_name).unwrap().clone();

        let vao = unsafe { VertexArray::new().unwrap() };
        
        UiRenderer {
            width,
            height,
            shader_program,
            vao,
            node,
        }
    }

    pub fn initialize(&mut self) {
        self.apply_2d_matrix();
        self.create_quad_vao();
    }

    fn apply_2d_matrix(&mut self) {
        let projection: glm::Mat4 = glm::ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        let view: glm::Mat4 = glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, 1.0));
        
        unsafe {
            self.shader_program.apply();
            self.shader_program.set_mat4("projection", &projection);
            self.shader_program.set_mat4("view", &view);
        }
    }

    fn create_quad_vao(&self) {
        let vertices: [f32; 12] = [ 
            1.0, 1.0,
            1.0, -1.0,
            -1.0, -1.0,
    
            1.0, 1.0,
            -1.0, -1.0,
            -1.0, 1.0,
        ];

        let vbo: Buffer;
        unsafe {
            vbo = Buffer::new(gl::ARRAY_BUFFER).unwrap();
    
            self.vao.bind();
            vbo.bind(&vertices, gl::STATIC_DRAW);
            
            self.vao.set_attrb::<f32>(0, 2, 2, 0);
    
            vbo.unbind();
            self.vao.unbind();
        }
    }

    pub fn update(&mut self, _dt: f64) {
        let mut model: glm::Mat4 = glm::Mat4::identity();
        
        model = glm::translate(&model, &glm::vec3(self.node.transform.left, self.node.transform.top, 0.0));
        model = glm::scale(&model, &glm::vec3(self.node.transform.width / 2.0, self.node.transform.height / 2.0, 1.0));

        unsafe {
            self.shader_program.apply();

            self.shader_program.set_float("roundness", 10.0);
            self.shader_program.set_vec2("scale", &glm::vec2(self.node.transform.width / 2.0, self.node.transform.height / 2.0));
            self.shader_program.set_vec4("color", &self.node.transform.background_color.into_glm_vec4());
            self.shader_program.set_mat4("model", &model);
        }
    }

    pub unsafe fn draw(&mut self) {
        self.shader_program.apply();

        self.vao.bind();
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        self.vao.unbind();
    }
}