use bevy::prelude::*;
use lightyear::prelude::{
    NetworkTarget, Replicated, ServerConnectionManager, ServerReplicate,
    server::{AuthorityPeer, ReplicationTarget, SyncTarget},
};
use pha_common::Rendered;
use pha_protocol::component::{Player, ViewDirection};

pub(crate) struct ViewDirectionPlugin;

impl Plugin for ViewDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_server_view_direction_added);
    }
}

/* message_registry: Res<MessageRegistry>,
    // we use an EventReader and not an event because the user might want to re-broadcast the inputs
    mut received_inputs: EventReader<ServerReceiveMessage<InputMessage<A>>>,
    connection_manager: Res<ConnectionManager>,
    // TODO: currently we do not handle entities that are controlled by multiple clients
    mut query: Query<Option<&mut InputBuffer<A>>>,
    mut commands: Commands,
    tick_manager: Res<TickManager>,
) {
    let tick = tick_manager.tick();
    received_inputs.read().for_each(|event| {
        let message = &event.message;
        let client_id = event.from;
        trace!(?client_id, action = ?A::short_type_path(), ?message.end_tick, ?message.diffs, "received input message");

        // TODO: or should we try to store in a buffer the interpolation delay for the exact tick
        //  that the message was intended for?
        if let Some(interpolation_delay) = message.interpolation_delay {
            // update the interpolation delay estimate for the client
            if let Ok(client_entity) = connection_manager.client_entity(client_id) {
                commands.entity(client_entity).insert(interpolation_delay);
            }
        }
*/

fn handle_server_view_direction_added(
    mut commands: Commands,
    query: Query<
        (Entity, &AuthorityPeer),
        (Added<ViewDirection>, With<Replicated>, With<ViewDirection>),
    >,
    q_players: Query<(Entity, &Player)>,
) {
    for (view_direction_entity, auth) in query.iter() {
        println!("received Added<ViewDirection> on server.");
        match auth {
            AuthorityPeer::Client(client_id) => {
                commands
                    .entity(view_direction_entity)
                    .insert(ServerReplicate {
                        target: ReplicationTarget {
                            target: NetworkTarget::AllExceptSingle(*client_id),
                        },
                        authority: AuthorityPeer::Client(*client_id), // The client has authority over this
                        sync: SyncTarget {
                            interpolation: NetworkTarget::AllExceptSingle(*client_id), // Interpolate on other clients
                            ..default()
                        },
                        ..Default::default()
                    });

                // find player entity with the correct client_id
                for (player_entity, player) in q_players.iter() {
                    if player.0 == *client_id {
                        println!("Found player entity on server to add ViewDirection to!");
                        commands
                            .entity(player_entity)
                            .add_child(view_direction_entity);
                    }
                }
            }
            AuthorityPeer::Server => {}
            AuthorityPeer::None => {}
        }
    }
}
