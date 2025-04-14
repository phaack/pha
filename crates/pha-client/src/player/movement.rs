use bevy::prelude::*;
use pha_common::player::move_player;

pub struct ClientMovementPlugin;

impl Plugin for ClientMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_player);
    }
}
