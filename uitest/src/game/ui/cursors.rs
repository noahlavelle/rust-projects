use bevy::{prelude::*, window::{CursorIcon, CustomCursor, CustomCursorImage}};

use crate::game::{GameState, assets::sprites::ui::UIAssets};

#[derive(Resource, Default)]
pub enum CursorType {
    #[default]
    Point,
    Interact,
}

impl CursorType {
    pub fn to_index(&self) -> usize {
        match self {
            CursorType::Point => 5,
            CursorType::Interact => 1,
        }
    }
}

pub struct UICursorsPlugin;

impl Plugin for UICursorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorType>();
        app.add_systems(OnEnter(GameState::Setup), init_cursor);
        app.add_systems(Update, update_cursor.run_if(resource_changed::<CursorType>));
    }
}

fn init_cursor(
    mut commands: Commands,
    ui_assets: Option<Res<UIAssets>>,
    window: Single<Entity, With<Window>>,
    cursor_type: Res<CursorType>,
) {
    let Some(ui_assets) = ui_assets else {
        return;
    };

    commands.entity(*window).insert(
        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: ui_assets.cursors.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: ui_assets.cursors_layout.clone(),
                index: cursor_type.to_index(),
            }),
            ..default()
        }))
    );
}

fn update_cursor (
    cursor_type: Res<CursorType>,
    mut query: Query<&mut CursorIcon, With<Window>>,
) {
    for mut cursor_icon in &mut query {
        if let CursorIcon::Custom(CustomCursor::Image(ref mut image)) = *cursor_icon 
            && let Some(mut texture_atlas) = image.texture_atlas.take() {
                texture_atlas.index = cursor_type.to_index();
                image.texture_atlas = Some(texture_atlas);
        };
    }
}

