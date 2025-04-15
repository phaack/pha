use avian3d::prelude::{PhysicsGizmoExt, PhysicsGizmos};
use bevy::prelude::*;
use lightyear::prelude::{
    NetworkTarget, Replicated, ServerConnectionManager, ServerReplicate,
    server::{AuthorityPeer, ReplicationTarget, SyncTarget},
};
use pha_common::{Rendered, Simulated};
use pha_protocol::{component::Player, components::look_orientation::LookOrientation};

pub(crate) struct ServerLookOrientationPlugin;

impl Plugin for ServerLookOrientationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_server_view_direction_added);
        app.add_systems(Update, draw_debug_line);
    }
}

fn draw_debug_line(
    mut gizmos: Gizmos<PhysicsGizmos>,
    q_look_orientation: Query<(Entity, &LookOrientation, &Parent), Rendered>,
    q_global_transform: Query<&GlobalTransform>,
) {
    for (_entity, view_dir, parent) in q_look_orientation.iter() {
        let mut origin = Vec3::default();

        if let Some(transform) = q_global_transform.get(parent.get()).ok() {
            origin = transform.translation();
        }

        let forward = LookOrientation::forward(view_dir);
        let end = origin + forward * 100.0;

        let color = Color::srgb_u8(170, 100, 50);

        gizmos.draw_line(origin, end, color);
    }
}

fn handle_server_view_direction_added(
    mut commands: Commands,
    query: Query<
        (Entity, &AuthorityPeer),
        (
            Added<LookOrientation>,
            With<Replicated>,
            With<LookOrientation>,
        ),
    >,
    q_players: Query<(Entity, &Player)>,
) {
    for (view_direction_entity, auth) in query.iter() {
        println!("received Added<LookOrientation> on server.");
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
                        println!("Found player entity on server to add LookOrientation to!");
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
