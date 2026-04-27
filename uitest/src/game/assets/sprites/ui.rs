use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct UIAssets {
    #[asset(path = "embedded://sprites/ui/grey.png")]
    pub primary_slice: Handle<Image>,
    #[asset(path = "embedded://sprites/ui/brown.png")]
    pub secondary_slice: Handle<Image>,
    #[asset(path = "embedded://sprites/ui/brown_pressed.png")]
    pub secondary_slice_pressed: Handle<Image>,

}
