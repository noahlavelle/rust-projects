use bevy::prelude::*;

use crate::game::assets::sprites::ui::UIAssets;

#[derive(Component, Default, Clone, Copy, PartialEq)]
pub enum UIButtonState {
    #[default]
    None,
    Hovered,
    Clicked,
}

#[derive(Default, PartialEq)]
pub enum UIButtonStyle {
    None,
    #[default]
    Primary,
}

#[derive(Component)]
#[require(Node, UIButtonState)]
pub struct UIButton {
    width: Val,
    height: Val,
    style: UIButtonStyle,
    last_interaction: Interaction,
}

impl UIButton {
    pub fn new() -> Self {
        UIButton {
            width: Val::Auto,
            height: Val::Auto,
            style: UIButtonStyle::default(),
            last_interaction: Interaction::None,
        }
    }

    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    pub fn with_style(mut self, style: UIButtonStyle) -> Self {
        self.style = style;
        self
    }
    pub fn full(self) -> Self {
        self.with_width(Val::Percent(100.0))
            .with_height(Val::Percent(100.0))
    }
}

pub struct UIButtonPlugin;

impl Plugin for UIButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (register_ui, update_state));
    }
}

fn register_ui(
    mut commands: Commands,
    button_elements: Populated<(Entity, &UIButton), Added<UIButton>>,
    ui_assets: Option<Res<UIAssets>>,
) {
    let Some(ui_assets) = ui_assets else {
        return;
    };

    for (entity, button) in button_elements.iter() {
        commands.entity(entity).insert((
            Node {
                width: button.width,
                height: button.height,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Button,
        ));

        if button.style == UIButtonStyle::Primary {
            commands.entity(entity).insert(
                ImageNode {
                    image: ui_assets.secondary_slice.clone(),
                    image_mode: NodeImageMode::Sliced(TextureSlicer {
                        border: BorderRect::all(3.0),
                        center_scale_mode: SliceScaleMode::Stretch,
                        sides_scale_mode: SliceScaleMode::Stretch,
                        ..default()
                    }),
                    ..default()
                });
        }
    }
}

fn update_state(
    mut button_elements: Populated<(&Interaction, &mut UIButtonState, &mut UIButton, &mut ImageNode), (Changed<Interaction>, With<UIButton>)>,
    ui_assets: Option<Res<UIAssets>>,
) {
    let Some(ui_assets) = ui_assets else {
        return;
    };

    for (interaction, mut new_state, mut button, mut image_node) in button_elements.iter_mut() {
        // Only trigger click on release
        if *interaction == Interaction::Hovered && button.last_interaction == Interaction::Pressed {
            *new_state = UIButtonState::Clicked;
            image_node.image = ui_assets.secondary_slice.clone();
        } else {
            match *interaction {
                Interaction::Pressed => {
                    *new_state = UIButtonState::Hovered;
                    image_node.image = ui_assets.secondary_slice_pressed.clone();
                },
                Interaction::Hovered => {
                    *new_state = UIButtonState::Hovered;
                    image_node.image = ui_assets.secondary_slice.clone();
                },
                Interaction::None => {
                    *new_state = UIButtonState::None;
                    image_node.image = ui_assets.secondary_slice.clone();
                },
            };
        }
        button.last_interaction = *interaction;
    }
}

