extern crate nalgebra_glm as glm;

use std::collections::HashMap;
use crate::utility::standalone_functions::get_cstring;

#[derive(Clone)]
pub struct ShaderProgram {
    pub id: u32,
    shaders: HashMap<gl::types::GLenum, u32>
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
            for (_, value) in &self.shaders {
                gl::DeleteShader(*value);
            }
        }
    }
}

impl ShaderProgram {
    pub unsafe fn new(shader_codes: &[(&str, gl::types::GLenum)]) -> Result<Self, std::io::Error> {
        let mut program = Self { 
            id: gl::CreateProgram(),
            shaders: HashMap::new()
        };

        for &(shader_data, shader_type) in shader_codes {
            let shader = program.create_shader(shader_data, shader_type);
            program.compile_errors(shader, "SHADER");
            program.shaders.insert(shader_type, shader);

            gl::AttachShader(program.id, shader);
        }

        gl::LinkProgram(program.id);
        program.compile_errors(program.id, "PROGRAM");

        for (_, value) in &program.shaders {
            gl::DetachShader(program.id, *value);
        }

        Ok(program)
    }

    unsafe fn create_shader(&mut self, shader_data: &str, shader_type: gl::types::GLenum) -> u32 {
        let shader = gl::CreateShader(shader_type);

        gl::ShaderSource(shader, 1, &shader_data.as_bytes().as_ptr().cast(), &shader_data.len().try_into().unwrap());
        gl::CompileShader(shader);
        shader
    }

    pub unsafe fn apply(&mut self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_int(&self, name: &str, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &str, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), value);
    }

    pub unsafe fn set_vec2(&self, name: &str, value: &glm::Vec2) {
        gl::Uniform2fv(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), 1, &value[0]);
    }

    pub unsafe fn set_vec2f(&self, name: &str, x: f32, y: f32) {
        gl::Uniform2f(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), x, y);
    }

    pub unsafe fn set_vec3(&self, name: &str, value: &glm::Vec3) {
        gl::Uniform3fv(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), 1, &value[0]);
    }

    pub unsafe fn set_vec3f(&self, name: &str, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), x, y, z);
    }

    pub unsafe fn set_vec4(&self, name: &str, value: &glm::Vec4) {
        gl::Uniform4fv(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), 1, &value[0]);
    }

    pub unsafe fn set_vec4f(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        gl::Uniform4f(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), x, y, z, w);
    }

    pub unsafe fn set_mat2(&self, name: &str, mat: &glm::Mat2) {
        gl::UniformMatrix2fv(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), 1, gl::FALSE, &mat[0]);
    }

    pub unsafe fn set_mat3(&self, name: &str, mat: &glm::Mat3) {
        gl::UniformMatrix3fv(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), 1, gl::FALSE, &mat[0]);
    }

    pub unsafe fn set_mat4(&self, name: &str, mat: &glm::Mat4) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, get_cstring(name).as_ptr()), 1, gl::FALSE, &mat[0]);
    }

    unsafe fn compile_errors(&mut self, shader: u32, error_type: &str) {
        let mut success = 0;

        let mut log_len = 0_i32;
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        if error_type == "PROGRAM" {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
        }
        else {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("{} Shader Compile Error: {}", error_type, String::from_utf8_lossy(&v));
            }
        }
        
    }
}
