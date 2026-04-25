mod game;
mod consts;

use bevy::{
    prelude::*,
    window::{WindowResolution, PresentMode}
};
use bevy::app::ScheduleRunnerPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use consts::*;
use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.into(),
                        name: Some(WINDOW_TITLE.into()),
                        present_mode: PresentMode::AutoVsync,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                        resize_constraints: WindowResizeConstraints {
                            min_width: WINDOW_WIDTH,
                            min_height: WINDOW_HEIGHT,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default()),
            EmbeddedAssetPlugin::default(),
            GamePlugin
        ))
        .run();
}
