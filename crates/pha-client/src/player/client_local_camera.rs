use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use pha_protocol::input::NetworkedInput;
use pha_render::camera::DefaultCamera;

use crate::{game_state::GameState, input::LocalInput, replication::LocalPlayer};

pub struct LocalPlayerCameraPlugin;

impl Plugin for LocalPlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), enable_default_camera);
        app.add_systems(OnEnter(GameState::MainMenu), enable_default_camera);
        app.add_systems(OnEnter(GameState::Playing), disable_default_camera);

        // app.add_systems(FixedUpdate, rotate_camera_with_mouse_input);
    }
}

// camera management
fn disable_default_camera(mut q_camera: Query<&mut Camera, With<DefaultCamera>>) {
    println!("[PHA]: Disabled local default camera");
    let cam = q_camera.get_single_mut().ok();

    if let Some(mut c) = cam {
        c.is_active = false;
    }
}

fn enable_default_camera(mut q_camera: Query<&mut Camera, With<DefaultCamera>>) {
    println!("[PHA]: Enabled local default camera");
    let cam = q_camera.get_single_mut().ok();

    if let Some(mut c) = cam {
        c.is_active = true;
    }
}

// fn rotate_camera_with_mouse_input(
//     mut q_camera: Query<&mut Transform, (With<Camera3d>, With<LocalCameraMarker>)>,
//     q_input: Query<&ActionState<NetworkedInput>, With<LocalPlayer>>,
//     time: Res<Time>,
// ) {
//     for input in q_input.iter() {}
// }
