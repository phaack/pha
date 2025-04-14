use bevy::prelude::*;

use super::{movement::ClientMovementPlugin, client_look_orientation::ViewDirectionPlugin};

pub struct ClientPlayerPlugin;

impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ViewDirectionPlugin);
        app.add_plugins(ClientMovementPlugin);
    }
}
