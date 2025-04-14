use bevy::prelude::*;
use lightyear::prelude::{
    NetworkTarget, Replicated, ServerConnectionManager, ServerReplicate,
    server::{AuthorityPeer, ReplicationTarget, SyncTarget},
};
use pha_common::Rendered;
use pha_protocol::component::{Player, ViewDirection};

pub fn handle_client_view_direction_added(
    mut commands: Commands,
    query: Query<
        (Entity, &AuthorityPeer, &ViewDirection),
        (Added<ViewDirection>, With<Replicated>),
    >,
    client_entity_map: Res<ServerConnectionManager>,
) {
    for (entity, auth, view_direction) in query.iter() {
        println!("received Added<ViewDirection>");
        match auth {
            AuthorityPeer::Client(client_id) => {
                println!("ViewDirection on Server Entity belongs to {:?}", client_id);
                commands.entity(entity).insert(ServerReplicate {
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
            }
            AuthorityPeer::Server => {}
            AuthorityPeer::None => {}
        }
    }
}
