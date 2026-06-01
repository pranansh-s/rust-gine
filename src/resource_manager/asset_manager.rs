extern crate ini;

use std::fs::File;
use std::io::Read;
use ini::ini;
use std::collections::HashMap;

use crate::graphics::texture2d::{Texture2D, TextureAlpha};
use crate::graphics::shader_program::ShaderProgram;
use crate::resource_manager::map::Map;

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    shader_programs: HashMap<String, ShaderProgram>,
    maps: HashMap<String, Map>,
}

impl AssetManager {
    pub fn new() -> Self {
        AssetManager {
            textures: HashMap::new(),
            shader_programs: HashMap::new(),
            maps: HashMap::new(),
        }
    }

    pub fn get_map(&mut self, name: &str) -> Option<&mut Map> {
        self.maps.get_mut(name)
    }

    pub fn load_map(&mut self, file: &str, name: &str) -> &mut Map {
        let map = self.load_map_files(file);
        self.maps.insert(name.to_string(), map);
        self.maps.get_mut(name).unwrap()
    }

    fn load_map_files(&self, name: &str) -> Map {
        let map_file = ini!(safe name).unwrap();
        let map_string: Vec<char> = map_file["map"]["data"].as_ref().unwrap().chars().collect();

        let width: u32 = map_file["map"]["width"].as_ref().unwrap().parse().unwrap();
        let height: u32 = map_file["map"]["height"].as_ref().unwrap().parse().unwrap();

        let data: Vec<Vec<char>> = map_string.chunks(width as usize).map(|chunk| chunk.to_vec()).collect();

        let mut texture_mapping: HashMap<String, String> = HashMap::new();
        for (name, texture_path) in map_file["tile"].iter() {
            if let Some(texture_path) = texture_path {
                texture_mapping.insert(name.to_string(), texture_path.to_string());
            }
        }

        Map::new(width, height, data, texture_mapping)
    }

    pub fn get_shader_program(&mut self, name: &str) -> Option<&mut ShaderProgram> {
        self.shader_programs.get_mut(name)
    }

    pub fn load_shader_program(&mut self, vertex_shader: &str, fragment_shader: &str, geometry_shader: Option<&str>, name: &str) -> &mut ShaderProgram {
        let shader_program = self.load_shader_program_files(vertex_shader, fragment_shader, geometry_shader);
        self.shader_programs.insert(name.to_string(), shader_program);
        self.shader_programs.get_mut(name).unwrap()
    }

    fn load_shader_program_files(&self, vertex_shader: &str, fragment_shader: &str, geometry_shader: Option<&str>) -> ShaderProgram {
        let mut vertex_file = File::open(vertex_shader).unwrap();
        let mut fragment_file = File::open(fragment_shader).unwrap();

        let mut vertex_data = String::new();
        vertex_file.read_to_string(&mut vertex_data).unwrap();

        let mut fragment_data = String::new();
        fragment_file.read_to_string(&mut fragment_data).unwrap();
        
        let geometry_data = geometry_shader.map(|value| {
            let mut geometry_file = File::open(value).unwrap();
            let mut geometry_data = String::new();
            geometry_file.read_to_string(&mut geometry_data).unwrap();
            geometry_data
        });

        unsafe {
            match geometry_data {
                Some(value) => ShaderProgram::new(&[(&vertex_data, gl::VERTEX_SHADER), (&fragment_data, gl::FRAGMENT_SHADER), (&value, gl::GEOMETRY_SHADER)]).unwrap(),
                None => ShaderProgram::new(&[(&vertex_data, gl::VERTEX_SHADER), (&fragment_data, gl::FRAGMENT_SHADER)]).unwrap(),
            }
        }
    }

    pub fn get_texture(&mut self, name: &str) -> Option<&mut Texture2D> {
        self.textures.get_mut(name)
    }

    pub fn load_texture(&mut self, file: &str, name: &str, alpha: bool) -> &mut Texture2D {
        let texture = self.load_texture_file(file, alpha);
        self.textures.insert(name.to_string(), texture);
        self.textures.get_mut(name).unwrap()
    }

    fn load_texture_file(&self, file: &str, alpha: bool) -> Texture2D {
        let new_texture_file = image::open(file).expect("Failed to load texture file");
        let new_texture_data: TextureAlpha;

        if alpha {
            new_texture_data = TextureAlpha::RGBA(new_texture_file.into_rgba8());
        } else {
            new_texture_data = TextureAlpha::RGB(new_texture_file.into_rgb8());
        }

        match new_texture_data {
            TextureAlpha::RGBA(value) => {
                let (width, height) = value.dimensions();
                unsafe {
                    Texture2D::new(width, height, value.as_raw(), gl::RGBA as i32, gl::RGBA, gl::REPEAT as i32, gl::REPEAT as i32, gl::LINEAR as i32, gl::LINEAR as i32).unwrap()
                }
            }
            TextureAlpha::RGB(value) => {
                let (width, height) = value.dimensions();
                unsafe {
                    Texture2D::new(width, height, value.as_raw(), gl::RGB as i32, gl::RGB, gl::REPEAT as i32, gl::REPEAT as i32, gl::LINEAR as i32, gl::LINEAR as i32).unwrap()
                }
            }
        }
    }

    pub fn remove_map(&mut self, name: &str) -> Option<Map> {
        self.maps.remove(name)
    }

    pub fn insert_map(&mut self, name: &str, map: Map) {
        self.maps.insert(name.to_string(), map);
    }
}