extern crate nalgebra_glm as glm;

use std::collections::HashMap;
use crate::entity::sprite_renderer::SpriteRenderer;
use crate::entity::renderer::Renderer;
use crate::entity::Transform::Transform;
use crate::resource_manager::game_manager::TILE_SIZE;
use crate::resource_manager::asset_manager::AssetManager;

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec<char>>,
    tile_textures: HashMap<String, String>,
    renderer: Option<SpriteRenderer>,
}

impl Map {
    pub fn new(width: u32, height: u32, data: Vec<Vec<char>>, tile_textures: HashMap<String, String>) -> Self {
        Self {
            width,
            height,
            data,
            tile_textures,
            renderer: None,
        }
    }

    pub fn initialize(&mut self, (shader_path, program_name): (String, &str), view_width: f32, view_height: f32, manager: &mut AssetManager) {
        let terrain_vertex_shader = format!("shaders/{}/vertex.vs", shader_path);
        let terrain_fragment_shader = format!("shaders/{}/fragment.fs", shader_path);        

        manager.load_shader_program(&terrain_vertex_shader, &terrain_fragment_shader, None, program_name);
        for (name, path) in self.tile_textures.iter() {
            manager.load_texture(&path, name, false);
        }

        unsafe {
            let program = manager.get_shader_program(program_name).unwrap();

            let ortho: glm::Mat4 = glm::ortho(0.0, view_width, view_height, 0.0, -1.0, 1.0);
            program.apply();
            program.set_mat4("projection", &ortho);

            self.renderer = Some(SpriteRenderer::new(program.clone()));
        }
    }

    pub fn render(&mut self, manager: &mut AssetManager) {
        match self.renderer {
            Some(ref mut renderer) => {
                for (row_index, row) in self.data.iter().enumerate() {
                    for (col_index, &tile) in row.iter().enumerate() {
                        let texture_name = tile.to_string();
                        let tile_texture = manager.get_texture(texture_name.as_str()).unwrap();
                        let transform = Transform {
                            position: glm::vec2((col_index as f32) * TILE_SIZE, (row_index as f32) * TILE_SIZE),
                            scale: glm::vec2(TILE_SIZE, TILE_SIZE),
                            rotation: 0.0,
                        };
                        renderer.draw(tile_texture, transform);
                    }
                }
            }
            None => {
                println!("Initialise map first!");
            }
        }
    }
}