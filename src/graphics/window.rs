use glfw::{Action, Context, Key, WindowEvent, fail_on_errors, WindowHint};
use std::sync::mpsc::Receiver;

use crate::utility::Color::Color;

pub enum Click {
    LeftClick,
    RightClick,
    NoClick,
}

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
    color: Color,
    pub keys: [bool; glfw::ffi::KEY_LAST as usize + 1],
    pub mouse_scroll: f32,
    pub mouse_pos: (f32, f32),
    pub mouse_click: Click,
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.make_current();
        self.window.swap_buffers();
    }
}

impl Window {
    pub fn create(width: u32, height: u32, title: &str) -> Result<Self, String> {
        let mut glfw = glfw::init(fail_on_errors!()).map_err(|e| format!("GLFW Could not be initialised {:?} code:window.rs", e))?;
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::Resizable(false));
        
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).ok_or_else(|| String::from("Failed to create window code:window.rs"))?;
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);

        let color = Color(0.0, 0.0, 0.0, 1.0);
        let keys = [false; glfw::ffi::KEY_LAST as usize + 1];
        let mouse_scroll = 0.0;
        let mouse_pos = (0.0, 0.0);
        let mouse_click = Click::NoClick;

        Ok(Self {
            glfw,
            window,
            events,
            color,
            keys,
            mouse_scroll,
            mouse_pos,
            mouse_click,
        })
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    pub fn set_window_size(&mut self, width: i32, height: i32) {
        self.window.set_size(width, height);
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn get_window_time(&self) -> f64 {
        self.glfw.get_time()
    }

    pub fn clear_window(&self) {
        unsafe {
            gl::ClearColor(self.color.0, self.color.1, self.color.2, self.color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|s| self.window.get_proc_address(s) as *const _);

        //2D rendering blend alpha
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

    fn process_events(&mut self) { 
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(key, _, action, _) => {
                    if key == Key::Escape && action == Action::Press {
                        self.window.set_should_close(true);
                    }
                    else {
                        if action == Action::Press {
                            self.keys[key as usize] = true;
                        }
                        else if action == Action::Release {
                            self.keys[key as usize] = false;
                        }
                    }
                },
                glfw::WindowEvent::Scroll(_, y_offset) => {
                    self.mouse_scroll = y_offset as f32;
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    self.mouse_pos = (x as f32, y as f32);
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, action, _) => {
                    if action == glfw::Action::Press {
                        self.mouse_click = Click::LeftClick;
                    }
                    else if action == Action::Release {
                        self.mouse_click = Click::NoClick;
                    }
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButton::Button2, action, _) => {
                    if action == glfw::Action::Press {
                        self.mouse_click = Click::RightClick;
                    }
                    else if action == Action::Release {
                        self.mouse_click = Click::NoClick;
                    }
                }
                _ => {}
            }
        }

        if self.mouse_scroll != 0.0 {
            let decay_rate = 0.7;

            self.mouse_scroll -= self.mouse_scroll * decay_rate;
            if self.mouse_scroll.abs() < 0.01 {
                self.mouse_scroll = 0.0;
            }
        }
    }
}
