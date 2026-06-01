use engine::graphics::window::Window;
use engine::resource_manager::asset_manager::AssetManager;
use engine::ui::ui_component::Div;
use engine::ui::attribute::Attribute;
use engine::ui::ui_renderer::UiRenderer;
use engine::utility::Color::Color;

fn main() {
    let width = 800;
    let height = 600;

    let mut window = Window::create(width, height, "UI DOM Example").unwrap();
    window.init_gl();

    let mut asset_manager = AssetManager::new();

    let root_ui = Div::new(
        1,
        "background",
        vec![
            ("top", Attribute::Float(0.0)),
            ("left", Attribute::Float(0.0)),
            ("width", Attribute::Float(800.0)),
            ("height", Attribute::Float(600.0)),
            ("background-color", Attribute::Color(Color(0.15, 0.15, 0.18, 1.0))),
        ],
    );

    let mut ui_renderer = UiRenderer::new(
        "../assets/shaders/UiComponent",
        &root_ui,
        width,
        height,
        &mut asset_manager,
    );
    ui_renderer.initialize();

    let mut last_frame = window.get_window_time();

    while !window.should_close() {
        let current_time = window.get_window_time();
        let dt = current_time - last_frame;
        last_frame = current_time;

        ui_renderer.update(dt);

        window.clear_window();
        unsafe {
            ui_renderer.draw();
        }
        window.update();
    }
}
