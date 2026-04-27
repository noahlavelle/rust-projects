use bevy::prelude::*;
use crate::game::GameState;
use crate::game::ui::components::button::{UIButton, UIButtonState, UIButtonStyle};
use crate::game::ui::components::container::{UIContainer, UIContainerDisplay};
use crate::game::ui::components::container::close::UIContainerClose;
use crate::game::ui::components::container::title::UIContainerTitle;
use crate::game::ui::UIState;
use crate::game::ui::components::text::UIText;

const PANEL_WIDTH: f32 = 375.0;
const PANEL_HEIGHT: f32 = 450.0;

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
                    .with_width(Val::Px(PANEL_WIDTH))
                    .with_height(Val::Px(PANEL_HEIGHT))
                    .panel()
                    .center(),
                UIContainerDisplay::Panel,
            )).with_children(|root| {
                    root.spawn((
                        UIContainer::new().with_width(Val::Percent(100.0)).with_height(Val::Px(60.0)),
                    )).with_children(|root| {
                            root.spawn((ButtonAction::A, UIButton::new().full()))
                                .with_child(UIText::new().with_size(24.0).with_content("Action A".into()));
                        });

                    root.spawn((
                        UIContainer::new().with_width(Val::Percent(100.0)).with_height(Val::Px(60.0)),
                    )).with_children(|root| {
                            root.spawn((ButtonAction::B, UIButton::new().full()))
                                .with_child(UIText::new().with_size(24.0).with_content("Quit".into()));
                        });

                    root.spawn(UIContainerClose).with_child((UIButton::new().with_style(UIButtonStyle::None).full(), ButtonAction::Close));
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
                next_ui_state.set(UIState::Blank);
            },
            ButtonAction::A => println!("A Clicked"),
            ButtonAction::B => next_game_state.set(GameState::Ended),
        }
    }
}
