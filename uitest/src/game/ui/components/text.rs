use bevy::prelude::*;

use crate::game::assets::fonts::FontAssets;

#[derive(Default)]
pub enum FontStyle {
    #[default]
    Regular,
    Italic,
}

#[derive(Component)]
#[require(Node)]
pub struct UIText {
    content: String,
    color: Color,
    size: f32,
    font_style: FontStyle,
}

impl UIText {
    pub fn new() -> Self {
        UIText {
            content: String::default(),
            color: Color::BLACK,
            size: 12.0,
            font_style: FontStyle::default(),
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
    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
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
    font_assets: Option<Res<FontAssets>>,
) {
    let Some(font_assets) = font_assets else {
        return;
    };

    for (entity, text) in text_elements.iter() {
        let font = match text.font_style {
            FontStyle::Regular => font_assets.primary_font.clone(),
            FontStyle::Italic => font_assets.italic_font.clone(),
        };

        commands.entity(entity).insert((
            Text(text.content.clone()),
            TextColor(text.color),
            TextFont {
                font,
                font_size: text.size,
                ..default()
            }
        ));
    }
}
