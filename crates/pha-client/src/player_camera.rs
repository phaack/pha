use avian3d::prelude::Rotation;
use bevy::prelude::*;
use lightyear::prelude::ClientReplicate;
use pha_protocol::component::ViewDirection;
use pha_render::camera::DefaultCamera;

use crate::{game_state::GameState, replication::LocalPlayer};

#[derive(Component)]
pub(crate) struct LocalCamera;

pub(crate) struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), enable_default_camera);
        app.add_systems(OnEnter(GameState::MainMenu), enable_default_camera);
        app.add_systems(
            OnEnter(GameState::Playing),
            (disable_default_camera, add_player_camera),
        );

        app.add_systems(FixedUpdate, test_rotate_camera);
        app.add_systems(FixedUpdate, update_view_direction);
    }
}

fn add_player_camera(mut q_player: Query<Entity, With<LocalPlayer>>, mut commands: Commands) {
    let player_camera = commands
        .spawn(Camera3d::default())
        .insert(LocalCamera)
        // .insert(ClientReplicate {
        //     target: lightyear::prelude::client::ReplicateToServer,
        //     authority: lightyear::prelude::HasAuthority,
        //     replicating: lightyear::prelude::Replicating,
        //     ..Default::default()
        // })
        .id();
    for entity in q_player.iter() {
        commands.entity(entity).add_child(player_camera);
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

fn test_rotate_camera(
    mut q_camera: Query<(&mut Transform, &Camera), (With<Camera3d>, Without<DefaultCamera>)>,
    time: Res<Time>,
) {
    // only rotate every 2 seconds
    if time.elapsed_secs() % 2.0 < 0.01 {
        for (mut transform, _) in q_camera.iter_mut() {
            // rotate left, right
            transform.rotation = Quat::from_rotation_y(time.elapsed_secs());
        }
    }
}

fn update_view_direction(
    q_camera: Query<&Transform, (With<Camera3d>, With<LocalCamera>)>,
    mut q_player: Query<(Entity, &mut ViewDirection), With<LocalPlayer>>,
) {
    if let Ok(camera_transform) = q_camera.get_single() {
        for (_, mut view_direction) in q_player.iter_mut() {
            // Convert Bevy's Quat to avian3d's Quaternion
            let forward = camera_transform.forward();
            // expects two vectors
            // forward is a direction and not a vector
            let quat = Quat::from_rotation_arc(Vec3::Z, forward.into());

            // Update the ViewDirection
            *view_direction = ViewDirection(quat);
        }
    }
}
