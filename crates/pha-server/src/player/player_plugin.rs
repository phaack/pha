use bevy::prelude::*;

use super::{movement::MovementPlugin, server_look_orientation::ServerLookOrientationPlugin};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementPlugin);
        app.add_plugins(ServerLookOrientationPlugin);
    }
}
