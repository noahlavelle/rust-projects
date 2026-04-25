use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::game::GameState;
use crate::game::input::actions::InputAction;

pub mod actions;

#[derive(Eq, Hash, PartialEq)]
pub enum InputType {
    Scalar,
}

#[derive(Resource, Default)]
pub struct RawFrameKeys(Vec<KeyCode>);

#[derive(Resource)]
pub struct BaseKeyMap(pub HashMap<InputAction, Vec<KeyCode>>);

struct InputValues(HashMap<InputAction, ScalarInput>);

#[derive(Resource, Default)]
pub struct InputStore(HashMap<InputType, InputValues>);

impl InputStore {
    pub fn get_scalar(&self, input_action: InputAction) -> Option<&ScalarInput> {
        if let Some(scalar_store) = self.0.get(&InputType::Scalar) {
            scalar_store.0.get(&input_action)
        } else {
            None
        }
    }
}

pub struct ScalarInput {
    keys: Vec<KeyCode>,
}

impl ScalarInput {
    pub fn pressed(&self, frame_keys: &RawFrameKeys) -> bool {
        for key in self.keys.iter() {
            if frame_keys.0.contains(key) {
                return true
            }
        }
        false
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Starting), build_store);
        app.add_systems(Update, collect_frame_inputs);
        app.init_resource::<InputStore>();
        app.init_resource::<RawFrameKeys>();
    }
}

fn build_store(map: Res<BaseKeyMap>, mut input_store: ResMut<InputStore>) {
    let mut scalar_store = InputValues(HashMap::new());
    for action in map.0.keys() {
        scalar_store.0.insert(*action, ScalarInput { keys: map.0[action].clone() });
    }

    input_store.0 = HashMap::new();
    input_store.0.insert(InputType::Scalar, scalar_store);
}

fn collect_frame_inputs(keys: Res<ButtonInput<KeyCode>>, mut store: ResMut<RawFrameKeys>) {
    store.0 = keys.get_just_pressed().copied().collect();
}
