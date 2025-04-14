use bevy::prelude::*;
use pha_common::player::move_player;

pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_player);
    }
}
