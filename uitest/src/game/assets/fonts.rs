use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "embedded://fonts/noto_sans/NotoSans-Regular.ttf")]
    pub primary_font: Handle<Font>,
    #[asset(path = "embedded://fonts/noto_sans/NotoSans-Italic.ttf")]
    pub italic_font: Handle<Font>,
}
