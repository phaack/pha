use bevy::prelude::*;

use super::{movement::ClientMovementPlugin, view_direction::ViewDirectionPlugin};

pub struct ClientPlayerPlugin;

impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ViewDirectionPlugin);
        app.add_plugins(ClientMovementPlugin);
    }
}
