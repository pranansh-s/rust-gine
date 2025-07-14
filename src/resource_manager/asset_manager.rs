extern crate ini;

use std::fs::File;
use std::io::Read;
use ini::ini;

use std::collections::HashMap;

use crate::graphics::texture2d::{Texture2D, TextureAlpha};
use crate::graphics::shader_program::ShaderProgram;
use crate::resource_manager::map::Map;

pub struct AssetManager {
    textures: HashMap<&'static str, Texture2D>,
    shader_programs: HashMap<&'static str, ShaderProgram>,
    maps: HashMap<&'static str, Map>, //TODO: hashmap -> graph for better traversing
    //sounds...
}

impl AssetManager {
    fn new() -> Self {
        AssetManager {
            textures: HashMap::new(),
            shader_programs: HashMap::new(),
            maps: HashMap::new(),
        }
    }

    //MAPS
    pub fn get_map(&mut self, name: &str) -> Option<&mut Map> {
        self.maps.get_mut(name)
    }

    pub fn load_map(&mut self, file: &str, name: &'static str) -> &mut Map {
        let map = self.load_map_files(file);
        self.maps.insert(name, map);
        self.maps.get_mut(name).unwrap()
    }

    fn load_map_files(&self, name: &str) -> Map { //TODO: multiple textures for connecting 
        let map_file = ini!(safe name).unwrap();
        let map_string: Vec<char> = map_file["map"]["data"].as_ref().unwrap().chars().collect();

        let width: Result<u32, _> = map_file["map"]["width"].as_ref().unwrap().parse();
        let height: Result<u32, _> = map_file["map"]["height"].as_ref().unwrap().parse();

        let data: Vec<Vec<char>> = map_string.chunks(width.clone().unwrap() as usize).map(|chunk| chunk.to_vec()).collect();

        let mut texture_mapping: HashMap<String, String> = HashMap::new();
        for (name, texture_path) in map_file["tile"].iter() {
            if let Some(texture_path) = texture_path {
                texture_mapping.insert(name.to_string(), texture_path.to_string());
            }
        }

        Map::new(width.unwrap(), height.unwrap(), data, texture_mapping)
    }

    //SHADER_PROGRAMS
    pub fn get_shader_program(&mut self, name: &str) -> Option<&mut ShaderProgram> {
        self.shader_programs.get_mut(name)
    }

    pub fn load_shader_program(&mut self, vertex_shader: &str, fragment_shader: &str, geometry_shader: Option<&str>, name: &'static str) -> &mut ShaderProgram { //TODO: later add tesselation, compute, etc
        let shader_program = self.load_shader_program_files(vertex_shader, fragment_shader, geometry_shader);
        self.shader_programs.insert(name, shader_program);
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
            let shader_program = match geometry_data {
                Some(value) => ShaderProgram::new(&[(&vertex_data, gl::VERTEX_SHADER), (&fragment_data, gl::FRAGMENT_SHADER), (&value, gl::GEOMETRY_SHADER)]).unwrap(),
                None => ShaderProgram::new(&[(&vertex_data, gl::VERTEX_SHADER), (&fragment_data, gl::FRAGMENT_SHADER)]).unwrap(),
            };
            shader_program
        }

    }

    //TEXTURES
    pub fn get_texture(&mut self, name: &str) -> Option<&mut Texture2D> {
        self.textures.get_mut(name)
    }

    pub fn load_texture(&mut self, file: &str, name: &'static str, alpha: bool) -> &mut Texture2D {
        let texture = self.load_texture_file(file, alpha);
        self.textures.insert(name, texture);
        self.textures.get_mut(name).unwrap()
    }

    fn load_texture_file(&self, file: &str, alpha: bool) -> Texture2D {
        let new_texture_file = image::open(file).expect("Failed to load texture file");
        let new_texture_data: TextureAlpha;

        if alpha {
            new_texture_data = TextureAlpha::RGBA(new_texture_file.into_rgba8());
        }
        else {
            new_texture_data = TextureAlpha::RGB(new_texture_file.into_rgb8());
        }

        match new_texture_data {
            TextureAlpha::RGBA(value) => {
                let (width, height) = value.dimensions();
        
                unsafe {
                    let new_texture: Texture2D = Texture2D::new(width, height, value.as_raw(), gl::RGBA as i32, gl::RGBA, gl::REPEAT as i32, gl::REPEAT as i32, gl::LINEAR as i32, gl::LINEAR as i32).unwrap();
                    new_texture
                }
            }
            TextureAlpha::RGB(value) => {
                let (width, height) = value.dimensions();
        
                unsafe {
                    let new_texture: Texture2D = Texture2D::new(width, height, value.as_raw(), gl::RGB as i32, gl::RGB, gl::REPEAT as i32, gl::REPEAT as i32, gl::LINEAR as i32, gl::LINEAR as i32).unwrap();
                    new_texture
                }
            }
        }
    }
}

static mut ASSET_MANAGER: Option<AssetManager> = None;
pub fn get() -> &'static mut AssetManager {
    unsafe {
        ASSET_MANAGER.get_or_insert_with(|| AssetManager::new())
    }
}