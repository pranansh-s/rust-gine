use engine::graphics::window::Window;
use engine::resource_manager::asset_manager::AssetManager;
use engine::entity::Transform::Transform;
use engine::character::character::Character;
use engine::character::controller::Controller;

fn main() {
    let width = 800;
    let height = 600;

    let mut window = Window::create(width, height, "Character Example").unwrap();
    window.init_gl();

    let mut asset_manager = AssetManager::new();

    let ortho = nalgebra_glm::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
    let transform = Transform {
        position: nalgebra_glm::vec2(370.0, 270.0),
        scale: nalgebra_glm::vec2(60.0, 60.0 * 1.314453125),
        rotation: 0.0,
    };

    let mut player = Character::new(
        "player",
        "../../assets/shaders/Character/Player",
        "assets/character.png",
        ortho,
        transform,
        &mut asset_manager,
    );

    let mut last_frame = window.get_window_time();

    while !window.should_close() {
        let current_time = window.get_window_time();
        let dt = current_time - last_frame;
        last_frame = current_time;

        player.update_control(
            &window.keys,
            &engine::resource_manager::game_manager::GameState::Playing,
            (0.0, width as f32),
            (0.0, height as f32),
            dt,
        );
        player.update(dt);

        window.clear_window();
        player.animate(&mut asset_manager);
        window.update();
    }
}
