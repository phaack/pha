use bevy::prelude::*;

use crate::player::client_local_camera::create_local_player_camera_component;

use super::{client_local_camera::LocalPlayerCameraPlugin, movement::ClientMovementPlugin};

pub struct ClientPlayerPlugin;

impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClientMovementPlugin);
        app.add_plugins(LocalPlayerCameraPlugin);
    }
}

pub(crate) fn client_spawn_local_player_components(commands: &mut Commands, player_entity: Entity) {
    println!("Adding local player components");

    // Create the local camera stuff
    let local_cam = create_local_player_camera_component(commands);
    commands.entity(player_entity).add_child(local_cam);
}
