use engine::graphics::window::Window;
use engine::resource_manager::asset_manager::AssetManager;
use engine::camera::camera::Camera;
use engine::character::controller::Controller;

fn main() {
    let width = 800;
    let height = 600;

    let mut window = Window::create(width, height, "Map Example").unwrap();
    window.init_gl();

    let mut asset_manager = AssetManager::new();
    let mut camera = Camera::new();

    asset_manager.load_map("assets/maps/main.ini", "main");
    let mut map = asset_manager.remove_map("main").unwrap();
    map.initialize(
        ("../assets/shaders/Terrain".to_string(), "terrain_mesh"),
        width as f32,
        height as f32,
        &mut asset_manager,
    );
    asset_manager.insert_map("main", map);

    let mut last_frame = window.get_window_time();

    while !window.should_close() {
        let current_time = window.get_window_time();
        let dt = current_time - last_frame;
        last_frame = current_time;

        camera.update_mouse(
            window.mouse_pos,
            &window.mouse_click,
            window.mouse_scroll,
            &engine::resource_manager::game_manager::GameState::Playing,
            dt,
        );

        if let Some(terrain_program) = asset_manager.get_shader_program("terrain_mesh") {
            unsafe {
                terrain_program.apply();
                terrain_program.set_mat4("view", &camera.view_matrix);
            }
        }

        window.clear_window();
        
        let key = "main";
        if let Some(mut map_loaded) = asset_manager.remove_map(key) {
            map_loaded.render(&mut asset_manager);
            asset_manager.insert_map(key, map_loaded);
        }

        window.update();
    }
}
