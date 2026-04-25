pub mod player;
pub mod input;
pub mod ui;

use bevy::app::App;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::game::input::{BaseKeyMap, InputPlugin};
use crate::game::input::actions::InputAction;
use crate::game::player::{Player, PlayerPlugin};
use crate::game::ui::UIPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();

        app.add_plugins((
            PlayerPlugin,
            InputPlugin,
            UIPlugin,
        ));

        app.add_systems(OnEnter(GameState::Setup), setup)
            .add_systems(OnEnter(GameState::Starting), start)
            .add_systems(OnEnter(GameState::Paused), pause)
            .add_systems(OnExit(GameState::Paused), resume)
            .add_systems(OnEnter(GameState::Ended), end);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Setup,
    Starting,
    Running,
    Paused,
    Ended,
}

fn setup(mut commands: Commands, mut next_game_state: ResMut<NextState<GameState>>) {
    commands.spawn(Camera2d::default());

    let mut input_map = BaseKeyMap(HashMap::new());
    input_map.0.insert(InputAction::Pause, vec![KeyCode::Escape]);
    input_map.0.insert(InputAction::Resume, vec![KeyCode::Escape]);
    input_map.0.insert(InputAction::Exit, vec![KeyCode::Backspace]);

    commands.insert_resource(input_map);

    next_game_state.set(GameState::Starting);
}

fn start(mut commands: Commands, mut next_game_state: ResMut<NextState<GameState>>) {
    commands.spawn((
        Player,
        Sprite::default(),
        Transform {
            scale: Vec3::new(1000.0, 1000.0, 1.0),
            ..default()
        },
    ));

    next_game_state.set(GameState::Running);
}

fn pause() {

}

fn resume() {

}

fn end() {

}
