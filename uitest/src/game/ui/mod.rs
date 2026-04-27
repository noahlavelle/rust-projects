pub mod components;
pub mod views;
pub mod cursors;

use bevy::prelude::*;
use crate::game::GameState;
use crate::game::input::{InputStore, RawFrameKeys};
use crate::game::input::actions::InputAction;
use crate::game::ui::components::UIComponentsPlugin;
use crate::game::ui::cursors::{CursorType, UICursorsPlugin};
use crate::game::ui::views::UIViewsPlugin;

#[derive(States, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum UIState {
    #[default]
    None,
    Blank,
    Menu,
    Paused,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIViewsPlugin, UIComponentsPlugin, UICursorsPlugin));
        app.init_state::<UIState>();
        app.add_systems(OnEnter(GameState::Setup), setup);
        app.add_systems(
            Update,
            (
                update_blank.run_if(in_state(UIState::Blank)),
                update_paused.run_if(in_state(UIState::Paused)),
            )
        );
    }
}

#[inline]
fn setup(
    mut next_ui_state: ResMut<NextState<UIState>>,
) {
    next_ui_state.set(UIState::Blank);
}

fn update_blank(
    input_store: Res<InputStore>,
    frame_keys: Res<RawFrameKeys>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ui_state: ResMut<NextState<UIState>>,
) {
    if input_store.get_scalar(InputAction::Pause).unwrap().pressed(&frame_keys) {
        next_game_state.set(GameState::Paused);
        next_ui_state.set(UIState::Paused);
    }

}
fn update_paused(
    input_store: Res<InputStore>,
    frame_keys: Res<RawFrameKeys>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ui_state: ResMut<NextState<UIState>>,
    mut cursor_type: ResMut<CursorType>,
) {
    if input_store.get_scalar(InputAction::Resume).unwrap().pressed(&frame_keys) {
        next_game_state.set(GameState::Running);
        next_ui_state.set(UIState::Blank);
        *cursor_type = CursorType::Interact;
    }
}
