use bevy::prelude::*;

#[derive(Component)]
#[require(Node)]
pub struct UIText {
    content: String,
    color: Color,
    size: f32,
}

impl UIText {
    pub fn new() -> Self {
        UIText {
            content: String::default(),
            color: Color::BLACK,
            size: 12.0,
        }
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

pub struct UITextPlugin;

impl Plugin for UITextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, register_ui);
    }
}

fn register_ui(
    mut commands: Commands,
    text_elements: Populated<(Entity, &UIText), Added<UIText>>,
) {
    for (entity, text) in text_elements.iter() {
        commands.entity(entity).insert((
            Text(text.content.clone()),
            TextColor(text.color),
            TextFont {
                font_size: text.size,
                ..default()
            }
        ));
    }
}