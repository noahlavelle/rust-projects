pub mod sprites;
pub mod fonts;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState};

use crate::game::{GameState, assets::{fonts::FontAssets, sprites::ui::UIAssets}};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Setup)
                .load_collection::<UIAssets>()
                .load_collection::<FontAssets>(),
        );
    } 
}
