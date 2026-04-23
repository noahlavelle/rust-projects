use bevy::prelude::{Color, Val};

pub const GRID_TILES: i32 = 21; // Should be odd
pub const GAME_SURFACE_SQUARE_SIZE: f32 = 764.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 820.0;
pub const WINDOW_TITLE: &str = "Snake";

pub const BACKGROUND_COLOR: Color = Color::srgb(0.255, 0.255, 0.255);
pub const GAME_SURFACE_COLOR: Color = Color::srgb(0.764, 0.812, 0.635);
pub const PLAYER_COLOR: Color = Color::srgb(0.235, 0.239, 0.180);
pub const FOOD_COLOR: Color = Color::srgb(0.141, 0.149, 0.247);
pub const EMPTY_TILE_COLOR: Color = Color::srgb(0.663, 0.710, 0.553);
pub const TILE_TEXTURE_SIZE: f32 = 16.0;
pub const BORDER_THICKNESS: f32 = 24.0;

pub const SCOREBOARD_FONT_SIZE: f32 = 55.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(20.0);
pub const SCOREBOARD_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub const STARTING_LENGTH: i32 = 2; // Excluding head
pub const TIMESTEP_FREQUENCY: f64 = 9.0;

