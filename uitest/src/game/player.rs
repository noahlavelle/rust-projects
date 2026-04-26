use bevy::prelude::*;
use crate::game::GameState;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerStats {
    pub(crate) color: Color,
    pub(crate) speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { color: Color::hsl(180.0, 0.4, 0.4), speed: 50.0 }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>();

        app.add_systems(
            Update, (
                update_player.run_if(in_state(GameState::Running)),
                render_player.run_if(resource_changed::<PlayerStats>),
            )
        );
    }
}

fn update_player(
    mut player_stats: ResMut<PlayerStats>,
    time: Res<Time>,
) {
    let mut current_hsla: Hsla = player_stats.color.into();
    current_hsla.hue = (current_hsla.hue + time.delta_secs() * player_stats.speed) % 360.0;
    player_stats.color = current_hsla.into();
}

fn render_player(player_stats: ResMut<PlayerStats>, mut player: Single<&mut Sprite, With<Player>>) {
    player.color = player_stats.color;
}
