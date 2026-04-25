use bevy::prelude::*;
use crate::game::ui::components::container::{UIContainer, TITLE_FONT_SIZE};
use crate::game::ui::components::text::UIText;

#[derive(Component)]
#[require(Node)]
pub struct UIContainerTitle {
    content: String,
}

impl UIContainerTitle {
    pub fn from_text(text: String) -> Self {
        UIContainerTitle { content: text }
    }
}

pub struct UIContainerTitlePlugin;

impl Plugin for UIContainerTitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, register_ui);
    }
}
fn register_ui(
    mut commands: Commands,
    title_elements: Populated<(Entity, &UIContainerTitle), Added<UIContainerTitle>>,
) {
    for (entity, title) in title_elements.iter() {
        commands.entity(entity).insert((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(6.0),
                left: Val::Px(6.0),
                ..default()
            },
           UIText::new()
                .with_content(title.content.clone())
                .with_size(TITLE_FONT_SIZE)
        ));
    }
}
