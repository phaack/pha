use bevy::prelude::*;
use lightyear::prelude::client::ReplicateToServer;
use pha_protocol::component::ViewDirection;

use crate::{player_camera::LocalCamera, replication::LocalPlayer};

#[derive(Component)]
pub struct LocalViewDirection;

pub(crate) struct ViewDirectionPlugin;

impl Plugin for ViewDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_view_direction_from_camera);
    }
}

pub fn create_view_direction_component(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            ViewDirection::default(),
            LocalViewDirection,
            ReplicateToServer,
        ))
        .id()
}

fn update_view_direction_from_camera(
    q_camera: Query<&Transform, (With<Camera3d>, With<LocalCamera>)>,
    mut q_player: Query<(Entity, &mut ViewDirection), With<LocalViewDirection>>,
) {
    for cam in q_camera.iter() {
        for (_, mut view_direction) in q_player.iter_mut() {
            // Convert Bevy's Quat to avian3d's Quaternion
            let forward = cam.forward();
            // expects two vectors
            // forward is a direction and not a vector
            let quat = Quat::from_rotation_arc(Vec3::Z, forward.into());

            // Update the ViewDirection
            *view_direction = ViewDirection(quat);
        }
    }
}
