use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq)]
pub enum UIButtonState {
    #[default]
    None,
    Hovered,
    Clicked,
}

#[derive(Component)]
#[require(Node, UIButtonState)]
pub struct UIButton {
    width: Val,
    height: Val,
}

impl UIButton {
    pub fn new() -> Self {
        UIButton {
            width: Val::Auto,
            height: Val::Auto,
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
) {
    for (entity, button) in button_elements.iter() {
        commands.entity(entity).insert((
            Node {
                width: button.width,
                height: button.height,
                ..default()
            },
            Button,
            BackgroundColor(Color::BLACK),
        ));
    }
}

fn update_state(
    mut button_elements: Populated<(&Interaction, &mut UIButtonState), (Changed<Interaction>, With<UIButton>)>,
) {
    for (interaction, mut new_state) in button_elements.iter_mut() {
        *new_state = match *interaction {
            Interaction::Pressed => UIButtonState::Clicked,
            Interaction::Hovered => UIButtonState::Hovered,
            Interaction::None => UIButtonState::None
        };
    }
}

