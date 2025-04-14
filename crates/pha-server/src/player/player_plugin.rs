use bevy::prelude::*;

use super::{movement::MovementPlugin, view_direction::ViewDirectionPlugin};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementPlugin);
        // TODO: into common
        app.add_plugins(ViewDirectionPlugin);
    }
}
