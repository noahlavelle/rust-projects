use std::cmp::PartialEq;
use bevy::prelude::*;
use rand::RngExt;

const TIMESTEP_FREQUENCY: f64 = 9.0;
const WINDOW_WIDTH: f32 = 784.0;
const WINDOW_HEIGHT: f32 = 784.0;
const WINDOW_TITLE: &str = "Snake";
const GRID_TILES: i32 = 29; // Should be odd
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PLAYER_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const FOOD_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const STARTING_LENGTH: i32 = 3;
const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(10.0);
const SCOREBOARD_TEXT_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

#[derive(Resource)]
struct GridSize(f32);

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct ScoreboardUI;

fn main() {
    App::new()
        .insert_resource(GridSize(WINDOW_WIDTH / GRID_TILES as f32))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_hz(TIMESTEP_FREQUENCY))
        .insert_resource(Score(0))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                create_food.before(handle_input),
                handle_input,
                handle_movement.after(handle_input),
                handle_collision.after(handle_movement),
                position_to_translation.after(handle_movement),
                update_scoreboard.after(handle_collision),
            )
        )
        .run();
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_keys<'a, I>(keys: I) -> Option<Direction>
    where
        I: IntoIterator<Item = &'a KeyCode>,
    {
        for key in keys {
            if *key == KeyCode::KeyW {
                return Some(Direction::Up);
            } else if *key == KeyCode::KeyS {
                return Some(Direction::Down);
            } else if *key == KeyCode::KeyA {
                return Some(Direction::Left);
            } else if *key == KeyCode::KeyD {
                return Some(Direction::Right);
            }
        }
        None
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Component, Default)]
struct Segment;
#[derive(Component)]
struct Velocity(Direction);
#[derive(Component, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Food;
#[derive(Component)]
struct ResetOnDeath;

fn setup(mut window: Single<&mut Window>, mut commands: Commands) {
    window.resizable = false;
    window.resolution.set(WINDOW_WIDTH, WINDOW_HEIGHT);
    window.title = WINDOW_TITLE.to_string();

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
    let base_segment = create_segment(None, commands);
    commands.entity(base_segment).insert(Velocity(Direction::Right));
    for i in 0..STARTING_LENGTH {
        create_segment(Some(i + 1), commands);
    }
}

fn create_segment(offset: Option<i32>, commands: &mut Commands) -> Entity {
    let position = Position { x: -offset.unwrap_or(0), y: 0 };
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
    mut query: Query<&mut Velocity, With<Segment>>
) {
    let mut head = query.single_mut().expect("Missing lead segment");
    if let Some(new_direction) = Direction::from_keys(keyboard_input.get_pressed()) {
        if new_direction != head.0.opposite() {
            head.0 = new_direction;
        }
    }
}

fn handle_movement(
    mut head_query: Query<(&mut Position, &Velocity), With<Segment>>,
    mut body_query: Query<&mut Position, (With<Segment>, Without<Velocity>)>
) {
    // Get head position
    let mut head = head_query.single_mut().expect("Missing lead segment");

    // Shift tail segments
    let mut last_position = head.0.clone();
    for mut segment in body_query.iter_mut() {
        let original = segment.clone();
        segment.x = last_position.x;
        segment.y = last_position.y;
        last_position = original;
    }

    // Move head in current direction
    match &head.1.0 {
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
    head_query: Query<(&Position, &Velocity), With<Segment>>,
    body_query: Query<&Position, (With<Segment>, Without<Velocity>)>,
    food_query: Query<(Entity, &Position), With<Food>>,
    reset_query: Query<Entity, With<ResetOnDeath>>
) {
    let head = head_query.single().expect("Missing lead segment");
    let food = food_query.single().expect("Missing lead segment");
    if head.0 == food.1 {
        commands.entity(food.0).despawn();
        create_segment(None, &mut commands);
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
