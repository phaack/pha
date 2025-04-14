use avian3d::prelude::Rotation;
use bevy::prelude::*;
use pha_protocol::{component::Player, components::look_orientation::LookOrientation};

pub(crate) struct CommonViewDirectionPlugin;

impl Plugin for CommonViewDirectionPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(FixedUpdate, rotate_player_from_look_orientation);
    }
}

// pub fn rotate_player_from_look_orientation(
//     mut q_player: Query<(&mut Rotation, &Children), With<Player>>,
//     q_view_direction: Query<(Entity, &LookOrientation)>,
// ) {
//     for (mut rotation, children) in q_player.iter_mut() {
//         // find the ViewDirection child
//         for child in children.iter() {
//             if let Ok(view_direction) = q_view_direction.get(*child) {
//                 // Get the quaternion from ViewDirection and convert to Bevy's Quat if needed
//                 let view_quat: Quat = view_direction.1.0;
//                 // Only apply yaw rotation to keep the player model upright
//                 let (_, yaw, _) = view_quat.to_euler(EulerRot::YXZ);
//                 rotation.0 = LookOrientation::from_yaw_pitch(yaw, 0.0).into();
//             }
//         }
//     }
// }
