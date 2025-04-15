use avian3d::prelude::{DebugRender, PhysicsGizmoExt, PhysicsGizmos};
use bevy::prelude::*;
use lightyear::prelude::{
    HasAuthority, ParentSync, ReplicateHierarchy, Replicated, ReplicationGroup,
    client::ReplicateToServer,
};
use pha_protocol::components::look_orientation::LookOrientation;

use super::client_local_camera::LocalCameraMarker;

#[derive(Component)]
pub struct LocalViewDirection;

pub(crate) struct ViewDirectionPlugin;

impl Plugin for ViewDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_look_orienatation_from_camera);
    }
}

pub fn create_look_orientation_component(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            LookOrientation::default(),
            LocalViewDirection,
            ReplicateToServer,
            // HasAuthority,
            // ParentSync::default(),
        ))
        .id()
}

fn update_look_orienatation_from_camera(
    q_camera: Query<&Transform, (With<Camera3d>, With<LocalCameraMarker>)>,
    mut q_player: Query<(Entity, &mut LookOrientation), With<LocalViewDirection>>,
) {
    for cam in q_camera.iter() {
        for (_, mut view_direction) in q_player.iter_mut() {
            // Update the ViewDirection
            *view_direction = LookOrientation::look_at(cam.forward().as_vec3());
        }
    }
}
