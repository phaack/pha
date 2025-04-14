use avian3d::prelude::{DebugRender, PhysicsGizmoExt, PhysicsGizmos};
use bevy::prelude::*;
use lightyear::prelude::{
    ParentSync, ReplicateHierarchy, Replicated, ReplicationGroup, client::ReplicateToServer,
};
use pha_protocol::component::ViewDirection;

use crate::{player_camera::LocalCamera, replication::LocalPlayer};

#[derive(Component)]
pub struct LocalViewDirection;

pub(crate) struct ViewDirectionPlugin;

impl Plugin for ViewDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_view_direction_from_camera);
        app.add_systems(FixedUpdate, draw_debug_line);
    }
}

fn draw_debug_line(
    mut gizmos: Gizmos<PhysicsGizmos>,
    q_view_dir: Query<(Entity, &ViewDirection), With<Replicated>>,
    q_parent: Query<&GlobalTransform>,
) {
    for (_entity, view_dir) in q_view_dir.iter() {
        let origin = Vec3::default();

        // Convert the quaternion into a direction vector (e.g., forward = negative Z in Bevy)
        let forward = view_dir.0 * Vec3::NEG_Z;
        let end = (origin + forward) * 100.0;

        println!("Drawing line from {:?} to {:?}", origin, end);

        gizmos.draw_line(origin, end, Color::srgb_u8(0, 150, 50));
    }
    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 100.0, 0.0);
    let color = Color::srgb_u8(200, 10, 10);

    gizmos.draw_line(a, b, color);
}

pub fn create_view_direction_component(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            ViewDirection::default(),
            LocalViewDirection,
            ReplicateToServer,
            // ParentSync::default(),
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
