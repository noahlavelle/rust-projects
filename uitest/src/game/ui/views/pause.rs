use bevy::prelude::*;
use crate::game::GameState;
use crate::game::ui::components::button::{UIButton, UIButtonState};
use crate::game::ui::components::container::{UIContainer};
use crate::game::ui::components::container::close::UIContainerClose;
use crate::game::ui::components::container::title::UIContainerTitle;
use crate::game::ui::UIState;

#[derive(Component)]
struct UIRoot;

#[derive(Component, PartialEq)]
enum ButtonAction {
    Close,
    A,
    B,
}

pub struct PauseView;

impl Plugin for PauseView {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Paused), load_ui);
        app.add_systems(OnExit(UIState::Paused), destroy_ui);
        app.add_systems(Update, handle_interaction);
    }
}

fn load_ui(mut commands: Commands) {
    commands.spawn((
        UIRoot,
        UIContainer::new().full().center(),
        BackgroundColor(Color::BLACK.with_alpha(0.5)),
    )).with_children(|root| {
            root.spawn((
                UIContainer::new()
                    .with_width(Val::Px(300.0))
                    .with_height(Val::Px(300.0))
                    .panel()
                    .center(),
                BackgroundColor(Color::srgb(0.6, 0.3, 0.6)),
            )).with_children(|root| {
                    root.spawn((
                        UIContainer::new().with_width(Val::Percent(100.0)).with_height(Val::Px(60.0)),
                    )).with_child((UIButton::new().full(), ButtonAction::A));
                    root.spawn((
                        UIContainer::new().with_width(Val::Percent(100.0)).with_height(Val::Px(60.0)),
                    )).with_child((UIButton::new().full(), ButtonAction::B));
                    root.spawn(UIContainerClose).with_child((UIButton::new().full(), ButtonAction::Close));
                    root.spawn(UIContainerTitle::from_text("Paused...".into()));
                });
        });
}

fn destroy_ui(mut commands: Commands, root: Single<Entity, With<UIRoot>>) {
    commands.entity(*root).despawn();

}

fn handle_interaction(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ui_state: ResMut<NextState<UIState>>,
    interactions: Query<(&ButtonAction, &UIButtonState), (Changed<UIButtonState>, With<UIButton>)>,
) {
    for (action, state) in interactions.iter() {
        if *state != UIButtonState::Clicked {
            return
        }

        match *action {
            ButtonAction::Close => {
                next_game_state.set(GameState::Running);
                next_ui_state.set(UIState::None);
            },
            ButtonAction::A => println!("A Clicked"),
            ButtonAction::B => println!("B Clicked"),
        }
    }
}
