use avian3d::prelude::{Position, Rotation};
use bevy::prelude::*;
use pha_protocol::component::Player;

use crate::replication::LocalPlayer;

pub(crate) struct TestCameraPlugin;

impl Plugin for TestCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera);
    }
}

fn update_camera(
    mut q_cam: Query<(&mut Transform), With<Camera>>,
    q_player: Query<(&Position, &Rotation), With<LocalPlayer>>,
) {
    for mut transform in q_cam.iter_mut() {
        if let Ok(player) = q_player.get_single() {
            println!("player q");
            transform.rotation = player.1.0;
            transform.translation = player.0.0;
        }
    }
}
