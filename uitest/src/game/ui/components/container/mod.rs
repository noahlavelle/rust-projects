pub mod title;
pub mod close;

use bevy::prelude::*;
use crate::game::assets::sprites::ui::UIAssets;
use crate::game::ui::components::container::close::{UIContainerClosePlugin};
use crate::game::ui::components::container::title::UIContainerTitlePlugin;

const PANEL_PADDING: Val = Val::Px(24.0);

#[derive(Component, Default, PartialEq)]
pub enum UIContainerDisplay {
    #[default]
    None,
    Panel,
}

#[derive(Component, Clone, Copy)]
#[require(Node, UIContainerDisplay)]
pub struct UIContainer {
    position_type: PositionType,
    position: UiRect,
    width: Val,
    height: Val,
    align_items: AlignItems,
    justify_content: JustifyContent,
    padding: UiRect,
}

impl UIContainer {
    pub fn new() -> Self {
        UIContainer {
            position_type: PositionType::default(),
            position: UiRect::AUTO,
            width: Val::Auto,
            height: Val::Auto,
            align_items: AlignItems::default(),
            justify_content: JustifyContent::default(),
            padding: UiRect::ZERO,
        }
    }

    pub fn with_position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = position_type;
        self
    }
    pub fn with_position(mut self, position: UiRect) -> Self {
        self.position = position;
        self
    }
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    pub fn with_align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }
    pub fn with_justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }
    pub fn with_padding(mut self, rect: UiRect) -> Self {
        self.padding = rect;
        self
    }

    pub fn panel(self) -> Self {
        self.with_padding(UiRect::all(PANEL_PADDING))
            .with_position_type(PositionType::Relative)
    }
    pub fn center(self) -> Self {
        self.with_align_items(AlignItems::Center)
            .with_justify_content(JustifyContent::Center)
    }
    pub fn full(self) -> Self {
        self.with_width(Val::Percent(100.0))
            .with_height(Val::Percent(100.0))
    }
}

pub struct UIContainerPlugin;

impl Plugin for UIContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIContainerClosePlugin, UIContainerTitlePlugin));
        app.add_systems(Update, register_ui);
    }
}

fn register_ui(
    mut commands: Commands,
    container_elements: Populated<(Entity, &UIContainer, &UIContainerDisplay), Added<UIContainer>>,
    ui_assets: Option<Res<UIAssets>>,
) {
    let Some(ui_assets) = ui_assets else {
        return;
    };

    for (entity, container, display) in container_elements.iter() {
        if *display == UIContainerDisplay::Panel {
            container.with_padding(UiRect::all(PANEL_PADDING));
            commands.entity(entity).insert(ImageNode {
                image: ui_assets.primary_slice.clone(),
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(8.0),
                    center_scale_mode: SliceScaleMode::Stretch,
                    max_corner_scale: 4.0,
                    ..default()
                }),
                ..default()
            });
        }

        commands.entity(entity).insert(Node {
                display: Display::Flex,
                position_type: container.position_type,
                left: container.position.left,
                right: container.position.right,
                top: container.position.top,
                bottom: container.position.bottom,
                padding: container.padding,
                row_gap: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                width: container.width,
                height: container.height,
                align_items: container.align_items,
                justify_content: container.justify_content,
                ..default()
            }
        );
    }
}
