extern crate nalgebra_glm as glm;

use crate::graphics::window::Click;
use crate::resource_manager::asset_manager;
use crate::resource_manager::map::Map;
use crate::entity::Transform::Transform;
use crate::camera::camera::Camera;
use crate::character::character::Character;
use crate::character::controller::Controller;

pub const TILE_SIZE: f32 = 60.00;

pub enum GameState {
    InMenu,
    Playing,
    Paused,
}

pub struct GameManager<'a> {
    width: u32,
    height: u32,
    state: GameState,
    current_map: Option<&'a mut Map>,
    camera: Option<Camera>,
    player: Option<Character>
}

impl<'a> GameManager<'a> {
    fn new(width: u32, height: u32) -> Self {
        GameManager {
            width,
            height,
            state: GameState::Playing,
            current_map: None,
            camera: None,
            player: None
        }
    }

    pub fn initialize(&mut self) {
        let manager = asset_manager::get();

        let ortho: glm::Mat4 = glm::ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        let transform = Transform {
            position: glm::vec2(0.0, 0.0),
            scale: glm::vec2(TILE_SIZE, TILE_SIZE * 1.314453125),
            rotation: 0.0,
        };
        let player = Character::new("player", "Player", "character.png", ortho, transform);
        
        let map = manager.load_map("maps/main.ini", "main");
        map.initialize(("Terrain".to_string(), "terrain_mesh"), self.width as f32, self.height as f32);

        let mut camera = Camera::new();
        let horizontal = ((self.width) as f32 - (map.width as f32 * TILE_SIZE)) / 2.0;
        let vertical = ((self.height) as f32 - (map.height as f32 * TILE_SIZE)) / 2.0;
        camera.update_pos_by(glm::vec2(horizontal, vertical));
        
        self.player = Some(player);
        self.current_map = Some(map);
        self.camera = Some(camera);
    }

    pub fn update(&mut self, dt: f64) {
        let player_program = asset_manager::get().get_shader_program("player").unwrap();
        let terrain_program = asset_manager::get().get_shader_program("terrain_mesh").unwrap();

        if let Some(camera) = self.camera.as_mut() {            
            unsafe {
                terrain_program.apply();
                terrain_program.set_mat4("view", &camera.view_matrix);
                player_program.apply();
                player_program.set_mat4("view", &camera.view_matrix);
            }
        }
        
        self.player.as_mut().unwrap().update(dt);
    }

    pub fn process_input(&mut self, keys: &[bool; glfw::ffi::KEY_LAST as usize + 1], scroll: f32, pos: (f32, f32), click: &Click, dt: f64) {
        if let Some(player) = self.player.as_mut() {
            let width_constraints = (0.0, ((self.current_map.as_mut().unwrap().width as f32) - player.transform.scale.x / TILE_SIZE) * TILE_SIZE);
            let height_constraints = (0.0, ((self.current_map.as_mut().unwrap().height as f32) - player.transform.scale.y / TILE_SIZE) * TILE_SIZE);
            
            player.update_control(keys, &self.state, width_constraints, height_constraints, dt);
        }

        if let Some(camera) = self.camera.as_mut() {
            camera.update_mouse(pos, click, scroll, &self.state, dt);
            camera.update_control(keys, &self.state, (0.0, 0.0), (0.0, 0.0), dt);
        }
    }

    pub fn render(&mut self) {
        if let Some(current_map) = self.current_map.as_mut() {
            current_map.render();
        }
        
        if let Some(player) = self.player.as_mut() {
            player.animate();
        }

    }
}

static mut GAME_MANAGER: Option<GameManager<'static>> = None;

pub fn new(width: u32, height: u32) -> &'static mut GameManager<'static> {
    unsafe {
        GAME_MANAGER.get_or_insert_with(|| GameManager::new(width, height))
    }
}