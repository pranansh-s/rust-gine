extern crate nalgebra_glm as glm;

use crate::graphics::window::Click;
use crate::resource_manager::game_manager::GameState;
use crate::resource_manager::asset_manager::AssetManager;
use crate::character::controller::Controller;
use crate::entity::sprite_renderer::SpriteRenderer;
use crate::entity::Transform::Transform;
use crate::entity::renderer::Renderer;

use rand::Rng;

pub struct Character {
    pub id: u32,
    pub transform: Transform,
    pub velocity: glm::Vec2,
    pub direction: glm::Vec2,
    pub texture: String,
    pub renderer: SpriteRenderer,
}

impl Character {
    pub fn new(name: &str, shader_path: &str, texture_path: &str, ortho: glm::Mat4, transform: Transform, manager: &mut AssetManager) -> Self {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen();

        let character_vertex_shader = format!("shaders/Character/{}/vertex.vs", shader_path);
        let character_fragment_shader = format!("shaders/Character/{}/fragment.fs", shader_path);        

        manager.load_shader_program(&character_vertex_shader, &character_fragment_shader, None, name);
        manager.load_texture(texture_path, name, true);

        let renderer = unsafe {
            let program = manager.get_shader_program(name).unwrap();

            program.apply();
            program.set_mat4("projection", &ortho);

            SpriteRenderer::new(program.clone())
        };

        let velocity = glm::vec2(64.0, 64.0);
        let direction = glm::vec2(0.0, 0.0);

        Self {
            id,
            transform,
            velocity,
            direction,
            texture: name.to_string(),
            renderer,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.transform.position.x += self.velocity.x * self.direction.x * dt as f32;
        self.transform.position.y += self.velocity.y * self.direction.y * dt as f32;
    }

    pub fn animate(&mut self, manager: &mut AssetManager) {
        let texture = manager.get_texture(&self.texture).unwrap();
        let transform = self.transform.clone();
        self.renderer.draw(texture, transform);
    }
}

impl Controller for Character {
    fn update_control(&mut self, keys: &[bool; glfw::ffi::KEY_LAST as usize + 1], state: &GameState, width_constraints: (f32, f32), height_constraints: (f32, f32), _dt: f64) {
        match state {
            GameState::Playing => {
                if keys['D' as usize] && self.transform.position.x <= width_constraints.1 {
                    self.direction.x = 1.0;
                } else if keys['A' as usize] && self.transform.position.x >= width_constraints.0 {
                    self.direction.x = -1.0;
                } else {
                    self.direction.x = 0.0;
                }
        
                if keys['S' as usize] && self.transform.position.y <= height_constraints.1 {
                    self.direction.y = 1.0;
                } else if keys['W' as usize] && self.transform.position.y >= height_constraints.0 {
                    self.direction.y = -1.0;
                } else {
                    self.direction.y = 0.0;
                }
            }
            GameState::InMenu => {
                println!("Do Stuff Controls Menu");
            }
            GameState::Paused => {
                println!("Do Stuff Controls Paused");
            }
        }
    }

    fn update_mouse(&mut self, _pos: (f32, f32), _click: &Click, _scroll: f32, _state: &GameState, _dt: f64) {}
}