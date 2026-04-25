use bevy::prelude::*;

#[derive(Component)]
#[require(Node)]
pub struct UIButton {

}

impl UIButton {
    pub fn new() -> Self {
        UIButton {

        }
    }

    pub fn build(self) -> impl Bundle {

    }
}

pub struct UIButtonPlugin;

impl Plugin for UIButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, register_ui);
    }
}

fn register_ui(
    mut commands: Commands,
    query: Query<(Entity, &UIButton), Added<UIButton>>,
) {

}
