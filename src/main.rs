use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod ui;
mod simulator;

use resources::*;
use systems::*;
use ui::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "🏴‍☠️ Pirate FinOps Treasure Hunt".into(),
            resolution: (960., 720.).into(),
            canvas: Some("#game-canvas".into()),
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }));

    app.init_state::<GameState>()
        .insert_resource(GameData::new(Difficulty::Normal))
        .init_resource::<SelectedTile>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, menu_system.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Playing), (setup_game, setup_ui))
        .add_systems(
            Update,
            (
                handle_player_movement,
                auto_reveal_current_tile,
                process_tile_fix,
                update_game_state,
                update_ui,
                check_win_condition,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_game)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(Update, game_over_system.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
