use crate::resource_manager::game_manager::GameState;
use crate::graphics::window::Click;

pub trait Controller {
    fn update_control(&mut self, keys: &[bool; glfw::ffi::KEY_LAST as usize + 1], state: &GameState, width_constraints: (f32, f32), height_constraints: (f32, f32), dt: f64);
    fn update_mouse(&mut self, pos: (f32, f32), click: &Click, scroll: f32, state: &GameState, dt: f64);
}