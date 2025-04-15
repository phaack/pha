use crate::{
    game_state::GameState, player::client_player_plugin::client_spawn_local_player_components,
};
use pha_common::player::camera_rotation_plugin::PlayerCameraHolderMarker;

use bevy::prelude::*;
use lightyear::prelude::{
    client::{ClientCommandsExt, ClientConnection, NetClient, ReplicateToServer},
    server::{AuthorityPeer, ControlledBy, Lifetime, ReplicationTarget, SyncTarget},
    *,
};
use pha_assets::{CurrentLevel, LevelState};
use pha_protocol::{
    component::Player,
    components::camera::NetworkedCameraTransform,
    message::{ClientLevelLoadComplete, ServerWelcome, UnorderedReliable},
};

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_server_welcome.run_if(in_state(GameState::ConnectingRemote)),
                #[cfg(feature = "host")]
                on_server_welcome.run_if(in_state(GameState::ConnectingSelf)),
            ),
        );
        app.add_systems(Update, await_spawn.run_if(in_state(GameState::Spawning)));
        app.add_systems(OnEnter(LevelState::Loaded), on_assets_loaded);

        // Add a cleanup system to run when the player disconnects or changes states
        app.add_systems(OnExit(GameState::Playing), cleanup_local_player);
    }
}

/// Tag component to identify the local player
#[derive(Component)]
pub struct LocalPlayer;

/// Once finished loading the assets that the server requested the client to load
/// Signal the completion to the server
fn on_assets_loaded(mut commands: Commands, mut client: ResMut<ClientConnectionManager>) {
    commands.set_state(GameState::Spawning);

    if let Err(e) =
        client.send_message::<UnorderedReliable, ClientLevelLoadComplete>(&ClientLevelLoadComplete)
    {
        println!("unable to signal client level load complete due to {}", e);
        commands.disconnect_client();
    }
}

/// Respond to the welcome message from the server by initiating a load of the level requested
fn on_server_welcome(
    mut server_welcome_events: ResMut<Events<ClientReceiveMessage<ServerWelcome>>>,
    game_state: Res<State<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for ev in server_welcome_events.drain() {
        next_state.set(GameState::Loading);
        current_level.0 = ev.message.current_level;
    }
}

/// Cleanup any existing LocalPlayer entities to avoid duplicates
fn cleanup_local_player(mut commands: Commands, local_players: Query<Entity, With<LocalPlayer>>) {
    for entity in local_players.iter() {
        commands.entity(entity).remove::<LocalPlayer>();
    }
}

fn await_spawn(
    mut commands: Commands,
    q_spawned_player: Query<(Entity, &Player), (Added<Player>, Without<Replicated>)>,
    client: Res<ClientConnection>,
    existing_local_players: Query<Entity, With<LocalPlayer>>,
) {
    // Clean up any existing LocalPlayer entities first to avoid duplicates
    for entity in existing_local_players.iter() {
        commands.entity(entity).remove::<LocalPlayer>();
    }

    if let Some(player_entity) = q_spawned_player.get_single().ok() {
        let client_id = player_entity.1.0;

        commands.entity(player_entity.0).insert(LocalPlayer);

        let camera_holder = commands
            .spawn((
                NetworkedCameraTransform::default(),
                PlayerCameraHolderMarker,
                ServerReplicate {
                    target: ReplicationTarget {
                        target: NetworkTarget::AllExceptSingle(client_id),
                    },
                    authority: AuthorityPeer::Server,
                    sync: SyncTarget {
                        prediction: NetworkTarget::Single(client_id),
                        interpolation: NetworkTarget::AllExceptSingle(client_id),
                    },
                    controlled_by: ControlledBy {
                        target: NetworkTarget::Single(client_id),
                        lifetime: Lifetime::Persistent,
                    },
                    hierarchy: ReplicateHierarchy {
                        enabled: true,
                        recursive: true,
                    },
                    // marker: lightyear::prelude::Replicating,
                    ..Default::default()
                },
            ))
            .insert((
                Camera {
                    is_active: true,
                    ..Default::default()
                },
                Camera3d::default(),
            ))
            .id();

        commands.entity(player_entity.0).add_child(camera_holder);

        commands.set_state(GameState::Playing);
    }
}
