extern crate nalgebra_glm as glm;

use crate::graphics::window::Click;
use crate::resource_manager::asset_manager::AssetManager;
use crate::entity::Transform::Transform;
use crate::camera::camera::Camera;
use crate::character::character::Character;
use crate::character::controller::Controller;
use crate::utility::time::TimeSystem;

pub const TILE_SIZE: f32 = 60.00;

pub enum GameState {
    InMenu,
    Playing,
    Paused,
}

pub struct GameManager {
    width: u32,
    height: u32,
    pub state: GameState,
    pub asset_manager: AssetManager,
    pub time: TimeSystem,
    current_map: Option<String>,
    camera: Option<Camera>,
    player: Option<Character>,
}

impl GameManager {
    pub fn new(width: u32, height: u32) -> Self {
        GameManager {
            width,
            height,
            state: GameState::Playing,
            asset_manager: AssetManager::new(),
            time: TimeSystem::new(60.0),
            current_map: None,
            camera: None,
            player: None
        }
    }

    pub fn initialize(&mut self) {
        let ortho: glm::Mat4 = glm::ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        let transform = Transform {
            position: glm::vec2(0.0, 0.0),
            scale: glm::vec2(TILE_SIZE, TILE_SIZE * 1.314453125),
            rotation: 0.0,
        };
        
        let player = Character::new("player", "Player", "character.png", ortho, transform, &mut self.asset_manager);
        
        self.asset_manager.load_map("maps/main.ini", "main");
        let mut map = self.asset_manager.remove_map("main").unwrap();
        map.initialize(("Terrain".to_string(), "terrain_mesh"), self.width as f32, self.height as f32, &mut self.asset_manager);

        let mut camera = Camera::new();
        let horizontal = ((self.width) as f32 - (map.width as f32 * TILE_SIZE)) / 2.0;
        let vertical = ((self.height) as f32 - (map.height as f32 * TILE_SIZE)) / 2.0;
        camera.update_pos_by(glm::vec2(horizontal, vertical));
        
        self.asset_manager.insert_map("main", map);
        
        self.player = Some(player);
        self.current_map = Some("main".to_string());
        self.camera = Some(camera);
    }

    pub fn update(&mut self, dt: f64) {
        self.time.update();

        if let Some(camera) = self.camera.as_mut() {            
            unsafe {
                if let Some(terrain_program) = self.asset_manager.get_shader_program("terrain_mesh") {
                    terrain_program.apply();
                    terrain_program.set_mat4("view", &camera.view_matrix);
                }
                if let Some(player_program) = self.asset_manager.get_shader_program("player") {
                    player_program.apply();
                    player_program.set_mat4("view", &camera.view_matrix);
                }
            }
        }
        
        if let Some(player) = self.player.as_mut() {
            player.update(dt);
        }
    }

    pub fn process_input(&mut self, keys: &[bool; glfw::ffi::KEY_LAST as usize + 1], scroll: f32, pos: (f32, f32), click: &Click, dt: f64) {
        if let Some(player) = self.player.as_mut() {
            let mut width_constraints = (0.0, 0.0);
            let mut height_constraints = (0.0, 0.0);
            
            if let Some(map_key) = &self.current_map {
                let map = self.asset_manager.get_map(map_key);
                if let Some(m) = map {
                    width_constraints = (0.0, ((m.width as f32) - player.transform.scale.x / TILE_SIZE) * TILE_SIZE);
                    height_constraints = (0.0, ((m.height as f32) - player.transform.scale.y / TILE_SIZE) * TILE_SIZE);
                }
            }
            
            player.update_control(keys, &self.state, width_constraints, height_constraints, dt);
        }

        if let Some(camera) = self.camera.as_mut() {
            camera.update_mouse(pos, click, scroll, &self.state, dt);
            camera.update_control(keys, &self.state, (0.0, 0.0), (0.0, 0.0), dt);
        }
    }

    pub fn render(&mut self) {
        if let Some(map_key) = &self.current_map {
            let key = map_key.clone();
            if let Some(mut map) = self.asset_manager.remove_map(&key) {
                map.render(&mut self.asset_manager);
                self.asset_manager.insert_map(&key, map);
            }
        }
        
        if let Some(player) = self.player.as_mut() {
            player.animate(&mut self.asset_manager);
        }
    }
}