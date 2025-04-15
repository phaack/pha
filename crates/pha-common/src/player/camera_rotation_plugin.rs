use avian3d::prelude::Rotation;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use pha_protocol::{component::Player, input::NetworkedInput};

pub(crate) struct CommonCameraRotationPlugin;

impl Plugin for CommonCameraRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, rotate_player_from_mouse_movement);
    }
}

// - Transform (local offset from player)
//         - Camera:
//           - Camera
//           - Transform (local rotation)
//         - PlayerCameraMarker

#[derive(Component)]
pub struct PlayerCameraHolderMarker;

#[derive(Component)]
pub struct PlayerCameraMarker;

pub fn rotate_player_from_mouse_movement(
    mut q_player: Query<(&mut Transform, &Children), (With<PlayerCameraHolderMarker>)>,
    input: Query<(Entity, &ActionState<NetworkedInput>)>,
) {
    for (network_input_entity, input) in input.iter() {
        // if()
    }

    // if let Some(mouse_movement) = input.dual_axis_data(&NetworkedInput::MouseMove) {
    //     // Get the mouse X and Y movement values
    //     let mouse_delta_x = mouse_movement.pair.x;
    //     let mouse_delta_y = mouse_movement.pair.y;
    //
    //     // Apply the rotation to all cameras with LocalCamera component
    //     for mut camera_transform in q_camera.iter_mut() {
    //         // Create a horizontal rotation (around Y axis) based on mouse X movement
    //         let yaw_rotation = Quat::from_rotation_y(-mouse_delta_x * 0.001);
    //
    //         // Create a vertical rotation (around local X axis) based on mouse Y movement
    //         let pitch_rotation = Quat::from_rotation_x(-mouse_delta_y * 0.001);
    //
    //         // Apply rotations to camera transform
    //         // Horizontal rotation is applied to the global Y axis
    //         camera_transform.rotation = yaw_rotation * camera_transform.rotation;
    //
    //         // Vertical rotation is applied to the local X axis
    //         // We need to rotate around the local X axis by applying rotation after the current rotation
    //         camera_transform.rotation = camera_transform.rotation * pitch_rotation;
    //
    //         // Optionally: Clamp vertical rotation to prevent over-rotation
    //         // This requires decomposing and recomposing the quaternion
    //         let (mut pitch, mut yaw, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    //         yaw = yaw.clamp(-1.0, 1.0); // Clamp to approximately +/- 60 degrees
    //         camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, pitch, yaw, roll);
    //     }
    // }
}
