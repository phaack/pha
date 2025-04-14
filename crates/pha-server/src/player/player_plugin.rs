use avian3d::prelude::Position;
use bevy::prelude::*;
use lightyear::prelude::{
    ClientId, NetworkTarget, ReplicateHierarchy, ServerReplicate,
    server::{ControlledBy, Lifetime, SyncTarget},
};
use pha_common::REPLICATION_GROUP_PREDICTED;
use pha_protocol::component::Player;

use super::{movement::MovementPlugin, server_look_orientation::ServerLookOrientationPlugin};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementPlugin);
        app.add_plugins(ServerLookOrientationPlugin);
    }
}

pub(crate) fn server_spawn_player(
    player_start_position: Position,
    client_id: ClientId,
    mut commands: &mut Commands,
) {
    println!("Spawning Player on Server! - spawn_player()");
    commands.spawn((
        player_start_position,
        Player(client_id),
        ServerReplicate {
            group: REPLICATION_GROUP_PREDICTED,
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                lifetime: Lifetime::SessionBased,
            },
            sync: SyncTarget {
                prediction: NetworkTarget::Single(client_id),
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            hierarchy: ReplicateHierarchy {
                enabled: false,
                ..default()
            },
            ..Default::default()
        },
    ));
}
