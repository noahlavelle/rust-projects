mod consts;

use std::collections::VecDeque;
use bevy::prelude::*;
use rand::RngExt;
use consts::*;

#[derive(PartialEq, Default)]
enum Direction {
    #[default]
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(States,Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Starting,
    Running,
    Paused,
    GameOver,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::None => Direction::Left, // Snake should spawn away from this opposite
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Resource)]
struct GridSize(f32);
#[derive(Resource, Deref, DerefMut)]
struct Score(usize);
#[derive(Resource)]
struct TileFullSprite(Sprite);

#[derive(Component)]
struct ScoreboardUI;
#[derive(Component)]
struct Segment;
#[derive(Component, Default)]
struct Head {
    direction: Direction,
    direction_queue: VecDeque<Direction>,
}
#[derive(Component, PartialEq, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Food;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, grid_size: Res<GridSize>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font: asset_server.load("text_seven_segment.ttf"),
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(SCOREBOARD_TEXT_COLOR),
        ScoreboardUI,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font: asset_server.load("text_seven_segment.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCOREBOARD_TEXT_COLOR),
        )],
    ));
    commands.spawn((
        Text::new("Move: Arrows\nPause/Resume: Esc\nRetry: Enter"),
        TextFont {
            font: asset_server.load("text_seven_segment.ttf"),
            font_size: CONTROLS_FONT_SIZE,
            ..default()
        },
        TextColor(SCOREBOARD_TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top: CONTROLS_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
    ));

    commands.spawn((
        Sprite::from_color(GAME_SURFACE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, 0.0, -10.0),
            scale: Vec3::new(GAME_SURFACE_SQUARE_SIZE, GAME_SURFACE_SQUARE_SIZE, 1.0),
            ..default()
        }
    ));
    let border_size = GAME_SURFACE_SQUARE_SIZE + BORDER_THICKNESS;
    commands.spawn((
        Sprite::from_color(EMPTY_TILE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, 0.0, -20.0),
            scale: Vec3::new(border_size, border_size, 1.0),
            ..default()
        }
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("tile_empty.png"),
            custom_size: Some(Vec2::splat(GAME_SURFACE_SQUARE_SIZE)),
            color: EMPTY_TILE_COLOR,
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: grid_size.0 / TILE_TEXTURE_SIZE,
            },
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 100.0),
            ..default()
        },
    ));

    commands.insert_resource(TileFullSprite(Sprite {
        image: asset_server.load("tile_full.png"),
        custom_size: Some(Vec2::new(grid_size.0, grid_size.0)),
        ..default()
    }));
}

fn spawn_snake(mut commands: Commands, tile_full_sprite: Res<TileFullSprite>) {
    let mut sprite = tile_full_sprite.0.clone();
    sprite.color = PLAYER_COLOR;

    let base_segment = create_segment(Position { x: 0, y: 0 }, &sprite, &mut commands);
    commands.entity(base_segment).insert(Head::default());
    for i in 0..STARTING_LENGTH {
        create_segment(Position { x: -(i + 1), y: 0 }, &sprite, &mut commands);
    }

    commands.set_state(GameState::Running);
}

fn create_segment(position: Position, sprite: &Sprite, commands: &mut Commands) -> Entity {
    commands.spawn((
        Transform {
            ..default()
        },
        Segment,
        position,
        sprite.clone(),
    )).id()
}

fn handle_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head: Single<&mut Head, With<Segment>>
) {
    let direction: Option<Direction> = if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        Some(Direction::Up)
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        Some(Direction::Down)
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        Some(Direction::Left)
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        Some(Direction::Right)
    } else {
        None
    };

    if let Some(direction) = direction {
        let last_direction = head.direction_queue.back().unwrap_or(&head.direction);
        if direction != last_direction.opposite() {
            head.direction_queue.push_back(direction);
        }
    }
}

fn handle_movement(
    mut head: Single<(&mut Position, &mut Head), With<Segment>>,
    mut body_query: Query<&mut Position, (With<Segment>, Without<Head>)>
) {
    if head.1.direction == Direction::None && head.1.direction_queue.is_empty() {
        return;
    }

    let mut last_position = head.0.clone();
    for mut segment in body_query.iter_mut() {
        let original = segment.clone();
        segment.x = last_position.x;
        segment.y = last_position.y;
        last_position = original;
    }

    if !head.1.direction_queue.is_empty() {
        head.1.direction = head.1.direction_queue.pop_front().unwrap();
    }
    match head.1.direction {
        Direction::None => {},
        Direction::Up => {
            head.0.y += 1;
        }
        Direction::Down => {
            head.0.y -= 1;
        }
        Direction::Right => {
            head.0.x += 1;
        }
        Direction::Left => {
            head.0.x -= 1;
        }
    }
}

fn handle_death(
    mut commands: Commands,
    head: Single<&Position, With<Head>>,
    segments: Query<&Position, (With<Segment>, Without<Head>)>,
    asset_server: Res<AssetServer>
) {
    if segments.iter().collect::<Vec<_>>().contains(&*head)
        || head.x < -GRID_TILES / 2
        || head.y < -GRID_TILES / 2
        || head.x > GRID_TILES / 2
        || head.y > GRID_TILES / 2
    {
        commands.spawn((
            AudioPlayer::new(asset_server.load("die.mp3")),
            PlaybackSettings::DESPAWN,
        ));
        commands.set_state(GameState::GameOver);
    }
}

fn handle_eat(
    mut commands: Commands,
    mut score: ResMut<Score>,
    head: Single<&Position, With<Head>>,
    food: Single<(Entity, &Position), With<Food>>,
    tile_full_sprite: Res<TileFullSprite>,
    asset_server: Res<AssetServer>
) {
    let mut sprite = tile_full_sprite.0.clone();
    sprite.color = PLAYER_COLOR;

    if *head == food.1 {
        commands.entity(food.0).despawn();
        create_segment(head.clone(), &sprite, &mut commands);
        **score += 1;

        commands.spawn((
            AudioPlayer::new(asset_server.load("eat.mp3")),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn handle_spawn_food(
    mut commands: Commands,
    body_query: Query<&Position, With<Segment>>,
    food_query: Query<&Position, With<Food>>,
    tile_full_sprite: Res<TileFullSprite>,
) {
    if food_query.is_empty() {
        let segments = body_query.iter().collect::<Vec<_>>();
        let position = loop {
            let random = Position {
                x: rand::rng().random_range(0..GRID_TILES) - (GRID_TILES / 2),
                y: rand::rng().random_range(0..GRID_TILES) - (GRID_TILES / 2),
            };
            if !segments.contains(&&random) {
                break random;
            }
        };
        let mut sprite = tile_full_sprite.0.clone();
        sprite.color = FOOD_COLOR;
        commands.spawn((
            sprite,
            Transform::default(),
            Food,
            position,
        ));
    }
}

fn position_to_translation(grid_size: Res<GridSize>, mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            position.x.clamp(-(GRID_TILES / 2), GRID_TILES / 2) as f32 * grid_size.0,
            position.y.clamp(-(GRID_TILES / 2), GRID_TILES / 2) as f32 * grid_size.0,
            1.0
        );
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUI>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

fn reset(mut commands: Commands,  food: Query<Entity, With<Food>>,  segments: Query<Entity, With<Segment>>) {
    for food in food.iter() {
        commands.entity(food).despawn();
    }
    for segment in segments.iter() {
        commands.entity(segment).despawn();
    }
}

#[inline]
fn listen_for_pause(keyboard_input: Res<ButtonInput<KeyCode>>, asset_server: Res<AssetServer>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        play_audio_attention(&asset_server, &mut commands);
        commands.set_state(GameState::Paused);
    }
}

#[inline]
fn listen_for_resume(keyboard_input: Res<ButtonInput<KeyCode>>, asset_server: Res<AssetServer>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        play_audio_attention(&asset_server, &mut commands);
        commands.set_state(GameState::Running);
    }
}

#[inline]
fn listen_for_restart(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut commands: Commands
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        **score = 0;
        play_audio_attention(&asset_server, &mut commands);
        commands.set_state(GameState::Starting);
    }
}

#[inline]
fn play_audio_attention(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("blip.mp3")),
        PlaybackSettings::DESPAWN,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: WINDOW_TITLE.to_string(),
                    resizable: false,
                    resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                    ..default()
                }),
            ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .insert_resource(GridSize(GAME_SURFACE_SQUARE_SIZE / GRID_TILES as f32))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_hz(TIMESTEP_FREQUENCY))
        .insert_resource(Score(0))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_snake.run_if(in_state(GameState::Starting)),
                handle_movement_input.run_if(in_state(GameState::Running)),
                listen_for_pause.run_if(in_state(GameState::Running)),
                listen_for_resume.run_if(in_state(GameState::Paused)),
                listen_for_restart.run_if(in_state(GameState::GameOver)),
                reset.run_if(in_state(GameState::GameOver)),
            )
        )
        .add_systems(
            FixedUpdate,
            (handle_movement, (handle_death, handle_eat).after(handle_movement), handle_spawn_food.after(handle_eat))
                .run_if(in_state(GameState::Running))
        )
        .add_systems(
            FixedPostUpdate,
            (position_to_translation, update_scoreboard)
                .run_if(in_state(GameState::Running))
        )
        .run();
}
