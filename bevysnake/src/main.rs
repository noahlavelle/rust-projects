mod consts;

use std::collections::VecDeque;
use bevy::prelude::*;
use bevy::window::EnabledButtons;
use rand::RngExt;
use consts::*;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
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

#[derive(Component)]
struct ScoreboardUI;

#[derive(Component, Default)]
struct Segment;
#[derive(Component)]
struct Head {
    direction: Direction,
    direction_queue: VecDeque<Direction>,
}
#[derive(Component, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Food;
#[derive(Component)]
struct ResetOnDeath;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("Score: "),
        TextFont {
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
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCOREBOARD_TEXT_COLOR),
        )],
    ));

    create_starter_segments(&mut commands);
}

fn create_starter_segments(commands: &mut Commands) {
    let base_segment = create_segment(Position { x: 0, y: 0 }, commands);
    commands.entity(base_segment).insert(Head { direction: Direction::Right, direction_queue: VecDeque::new() });
    for i in 0..STARTING_LENGTH {
        create_segment(Position { x: -(i + 1), y: 0 }, commands);
    }
}

fn create_segment(position: Position, commands: &mut Commands) -> Entity {
    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            ..default()
        },
        Segment,
        ResetOnDeath,
        position,
    )).id()
}

fn handle_input(
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
    // Shift tail segments
    let mut last_position = head.0.clone();
    for mut segment in body_query.iter_mut() {
        let original = segment.clone();
        segment.x = last_position.x;
        segment.y = last_position.y;
        last_position = original;
    }

    // Move head in current direction
    if !head.1.direction_queue.is_empty() {
        head.1.direction = head.1.direction_queue.pop_front().unwrap();
    }
    match head.1.direction {
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

fn handle_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    head: Single<(&Position, &Head), With<Segment>>,
    food: Single<(Entity, &Position), With<Food>>,
    body_query: Query<&Position, (With<Segment>, Without<Head>)>,
    reset_query: Query<Entity, With<ResetOnDeath>>
) {
    if head.0 == food.1 {
        commands.entity(food.0).despawn();
        create_segment(head.0.clone(), &mut commands);
        **score += 1;
    }

    let segments = body_query.iter().collect::<Vec<_>>();
    if segments.contains(&head.0)
        || head.0.x < -GRID_TILES / 2
        || head.0.y < -GRID_TILES / 2
        || head.0.x > GRID_TILES / 2
        || head.0.y > GRID_TILES / 2
    {
        for entity in reset_query.iter() {
            commands.entity(entity).despawn();
        }
        **score = 0;
        create_starter_segments(&mut commands);
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUI>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

fn create_food(
    mut commands: Commands,
    body_query: Query<&Position, With<Segment>>,
    food_query: Query<&Position, With<Food>>
) {
    if !food_query.is_empty() {
        return;
    }

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
    commands.spawn((
        Sprite::from_color(FOOD_COLOR, Vec2::ONE),
        Transform::default(),
        Food,
        ResetOnDeath,
        position,
    ));
}

fn position_to_translation(grid_size: Res<GridSize>, mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            position.x as f32 * grid_size.0,
            position.y as f32 * grid_size.0,
            0.0
        );
        transform.scale = Vec3::splat(grid_size.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_string(),
                resizable: false,
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GridSize(WINDOW_WIDTH / GRID_TILES as f32))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_hz(TIMESTEP_FREQUENCY))
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (handle_movement, handle_collision).chain()
        )
        .add_systems(
            FixedPostUpdate,
            (position_to_translation, update_scoreboard)
        )
        .add_systems(Update, (handle_input, create_food))
        .run();
}

