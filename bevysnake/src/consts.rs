use bevy::prelude::{Color, Val};

pub const GRID_TILES: i32 = 29; // Should be odd
pub const WINDOW_WIDTH: f32 = 784.0;
pub const WINDOW_HEIGHT: f32 = 784.0;
pub const WINDOW_TITLE: &str = "Snake";

pub const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const PLAYER_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
pub const FOOD_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

pub const SCOREBOARD_FONT_SIZE: f32 = 33.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(10.0);
pub const SCOREBOARD_TEXT_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub const STARTING_LENGTH: i32 = 3;
pub const TIMESTEP_FREQUENCY: f64 = 9.0;
