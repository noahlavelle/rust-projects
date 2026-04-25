use bevy::prelude::*;
use crate::game::ui::views::pause::PauseView;

pub mod pause;

pub struct UIViewsPlugin;

impl Plugin for UIViewsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            PauseView
        );
    }
}