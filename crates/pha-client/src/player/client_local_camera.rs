use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use pha_render::camera::DefaultCamera;

use crate::{game_state::GameState, input::LocalInput, replication::LocalPlayer};

// - Transform (local offset from player)
//         - Camera:
//           - Camera
//           - Transform (local rotation)
//         - PlayerCameraMarker

#[derive(Component)]
pub(crate) struct LocalCameraHolderMarker;

#[derive(Component)]
pub(crate) struct LocalCameraMarker;

pub(crate) struct LocalPlayerCameraPlugin;

impl Plugin for LocalPlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), enable_default_camera);
        app.add_systems(OnEnter(GameState::MainMenu), enable_default_camera);
        app.add_systems(OnEnter(GameState::Playing), (disable_default_camera));

        app.add_systems(FixedUpdate, rotate_camera_with_mouse_input);
    }
}

// camera management
fn disable_default_camera(mut q_camera: Query<&mut Camera, With<DefaultCamera>>) {
    let cam = q_camera.get_single_mut().ok();

    if let Some(mut c) = cam {
        c.is_active = false;
    }
}

fn enable_default_camera(mut q_camera: Query<&mut Camera, With<DefaultCamera>>) {
    let cam = q_camera.get_single_mut().ok();

    if let Some(mut c) = cam {
        c.is_active = true;
    }
}

fn rotate_camera_with_mouse_input(
    mut q_camera: Query<&mut Transform, (With<Camera3d>, With<LocalCameraMarker>)>,
    q_input: Query<&ActionState<LocalInput>, With<LocalPlayer>>,
    time: Res<Time>,
) {
    for input in q_input.iter() {
        if let Some(mouse_movement) = input.dual_axis_data(&LocalInput::MouseMove) {
            // Get the mouse X and Y movement values
            let mouse_delta_x = mouse_movement.pair.x;
            let mouse_delta_y = mouse_movement.pair.y;

            // Apply the rotation to all cameras with LocalCamera component
            for mut camera_transform in q_camera.iter_mut() {
                // Create a horizontal rotation (around Y axis) based on mouse X movement
                let yaw_rotation = Quat::from_rotation_y(-mouse_delta_x * 0.001);

                // Create a vertical rotation (around local X axis) based on mouse Y movement
                let pitch_rotation = Quat::from_rotation_x(-mouse_delta_y * 0.001);

                // Apply rotations to camera transform
                // Horizontal rotation is applied to the global Y axis
                camera_transform.rotation = yaw_rotation * camera_transform.rotation;

                // Vertical rotation is applied to the local X axis
                // We need to rotate around the local X axis by applying rotation after the current rotation
                camera_transform.rotation = camera_transform.rotation * pitch_rotation;

                // Optionally: Clamp vertical rotation to prevent over-rotation
                // This requires decomposing and recomposing the quaternion
                let (mut pitch, mut yaw, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
                yaw = yaw.clamp(-1.0, 1.0); // Clamp to approximately +/- 60 degrees
                camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, pitch, yaw, roll);
            }
        }
    }
}

pub(crate) fn create_local_player_camera_component(commands: &mut Commands) -> Entity {
    let camera_holder = commands
        .spawn((Transform::default(), LocalCameraHolderMarker))
        .id();

    let local_camera = commands
        .spawn((Camera3d::default(), Transform::default(), LocalCameraMarker))
        .id();

    // attach local_camera to camera_holder
    commands.entity(camera_holder).add_child(local_camera);

    camera_holder
}
