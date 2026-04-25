use bevy::prelude::*;
use crate::game::ui::components::container::{UIContainer};
use crate::game::ui::components::text::UIText;

#[derive(Component)]
#[require(Node)]
pub struct UIContainerClose;

pub struct UIContainerClosePlugin;

impl Plugin for UIContainerClosePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, register_ui);
    }
}

fn register_ui(
    mut commands: Commands,
    close_elements: Populated<Entity, Added<UIContainerClose>>,
) {
    for entity in close_elements.iter() {
        commands.entity(entity).insert((
            Node {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                position_type: PositionType::Absolute,
                top: Val::Px(6.0),
                right: Val::Px(6.0),
                ..default()
            },
           UIText::new()
                .with_content("x".into())
                .with_size(16.0),
        ));
    }
}