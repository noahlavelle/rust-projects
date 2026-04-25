use bevy::prelude::*;
use crate::game::ui::components::container::UIContainerPlugin;
use crate::game::ui::components::text::UITextPlugin;

pub mod container;
pub mod text;
pub mod button;

pub struct UIComponentsPlugin;

impl Plugin for UIComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UITextPlugin, UIContainerPlugin));
    }
}