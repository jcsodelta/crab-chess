use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use ui_components_bevy::board::piece;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn board_piece_component(canvas_id: String) {
    let mut window = Window::default();
    window.canvas = Some(canvas_id.into());
    window.fit_canvas_to_parent = true;

    let window_plugin = WindowPlugin {
        primary_window: Some(window),
        exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        close_when_requested: true,
    };

    let plugins_group = DefaultPlugins::build(DefaultPlugins).set(window_plugin);

    App::new()
        .add_plugins(plugins_group)
        .add_systems(Startup, setup)
        .add_systems(Startup, piece::startup)
        .add_systems(Update, piece::update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.5625, 0.0, 0.0)),
        },
        ..default()
    });
}
