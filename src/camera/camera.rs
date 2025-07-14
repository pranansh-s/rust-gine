extern crate nalgebra_glm as glm;

use crate::camera::Panning::Panning;
use crate::graphics::window::Click;
use crate::resource_manager::game_manager::GameState;
use crate::character::controller::Controller;

pub struct Camera {
    pub view_matrix: glm::Mat4,
    pub position: glm::Vec2,
    panning: Panning,
    scale: glm::Vec2,
    zoom_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        let view_matrix = glm::Mat4::identity();
        let position = glm::vec2(0.0, 0.0);
        let panning = Panning {
            mode: false,
            last_pos: glm::vec2(0.0, 0.0),
            speed: 50.0,
        };
        let scale = glm::vec2(1.0, 1.0);
        let zoom_speed = 5.0;

        Self {
            view_matrix,
            position,
            panning,
            scale,
            zoom_speed,
        }
    }

    pub fn update_speed_by(&mut self, new_speed: f32) {
        self.panning.speed += new_speed;
    }

    pub fn update_zoom_speed(&mut self, new_zoom_speed: f32) {
        self.zoom_speed = new_zoom_speed;
    }

    pub fn update_pos_by(&mut self, camera_position: glm::Vec2) {
        self.view_matrix = glm::translate(&self.view_matrix, &glm::vec3(camera_position.x, camera_position.y, 0.0));
        self.position = camera_position;
    }

    pub fn update_scale_by(&mut self, camera_scale: glm::Vec2, at_pos: glm::Vec2) {
        self.view_matrix = glm::translate(&self.view_matrix, &glm::vec3(at_pos.x - self.position.x, at_pos.y - self.position.y, 0.0));
        self.view_matrix = glm::scale(&self.view_matrix, &glm::vec3(1.0 + camera_scale.x, 1.0 + camera_scale.y, 1.0));
        self.view_matrix = glm::translate(&self.view_matrix, &glm::vec3(self.position.x - at_pos.x, self.position.y - at_pos.y, 0.0));

        self.scale += camera_scale;
    }
}

impl Controller for Camera {
    fn update_control(&mut self, keys: &[bool; glfw::ffi::KEY_LAST as usize + 1], state: &GameState, _ : (f32, f32), _ : (f32, f32), _dt: f64) {
        match state {
            GameState::Playing => {
                if keys[glfw::Key::KpAdd as usize] {
                    if self.panning.speed + 2.0 < 10.0 {
                        self.update_speed_by(2.0);
                    }
                }
                else if keys[glfw::Key::KpSubtract as usize] {
                    if self.panning.speed - 2.0 > 0.0 {
                        self.update_speed_by(-2.0);
                    }
                }
            }
            GameState::InMenu => { println!("Do Stuff Controls Menu"); }
            GameState::Paused => { println!("Do Stuff Controls Paused"); }
        }
    }

    fn update_mouse(&mut self, pos: (f32, f32), click: &Click, scroll: f32, state: &GameState, dt: f64) {
        match state {
            GameState::Playing => {
                //Scaling
                if scroll > 0.0 {
                    self.update_scale_by(glm::vec2(1.0, 1.0) * self.zoom_speed * dt as f32, glm::vec2(pos.0, pos.1));
                }
                else if scroll < 0.0 {
                    self.update_scale_by(glm::vec2(-1.0, -1.0) * self.zoom_speed * dt as f32, glm::vec2(pos.0, pos.1));
                }

                //Panning
                let pos_vec = glm::vec2(pos.0, pos.1);
                
                if self.panning.mode {
                    self.update_pos_by((pos_vec - self.panning.last_pos) * self.panning.speed * 1.0 / self.scale.x * dt as f32);
                    self.panning.last_pos = pos_vec;
                }

                match click {
                    Click::LeftClick => {
                        if !self.panning.mode {
                            self.panning.last_pos = pos_vec;
                            self.panning.mode = true;
                        }
                    }
                    Click::RightClick | Click::NoClick => {
                        self.panning.mode = false;
                    }
                }
            }
            GameState::InMenu => { println!("Do Stuff Mouse Menu"); }
            GameState::Paused => { println!("Do Stuff Mouse Paused"); }
        }
    }
}