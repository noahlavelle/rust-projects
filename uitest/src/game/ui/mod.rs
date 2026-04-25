pub mod components;
pub mod views;

use bevy::prelude::*;
use crate::game::GameState;
use crate::game::input::{InputStore, RawFrameKeys};
use crate::game::input::actions::InputAction;
use crate::game::ui::components::UIComponentsPlugin;
use crate::game::ui::views::UIViewsPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIState {
    #[default]
    None,
    Menu,
    Paused,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIViewsPlugin, UIComponentsPlugin));
        app.init_state::<UIState>();
        app.add_systems(
            Update,
            (
                update_none.run_if(in_state(UIState::None)),
                update_paused.run_if(in_state(UIState::Paused)),
            )
        );
    }
}

fn update_none(
    input_store: Res<InputStore>,
    frame_keys: Res<RawFrameKeys>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ui_state: ResMut<NextState<UIState>>,
) {
    if input_store.get_scalar(InputAction::Pause).unwrap().pressed(&*frame_keys) {
        next_game_state.set(GameState::Paused);
        next_ui_state.set(UIState::Paused);
    }

}
fn update_paused(
    input_store: Res<InputStore>,
    frame_keys: Res<RawFrameKeys>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ui_state: ResMut<NextState<UIState>>,
) {
    if input_store.get_scalar(InputAction::Resume).unwrap().pressed(&*frame_keys) {
        next_game_state.set(GameState::Running);
        next_ui_state.set(UIState::None);
    }
}
