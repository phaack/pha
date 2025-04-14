use avian3d::prelude::{Collider, LinearVelocity, RigidBody, Rotation, collider};
use bevy::{gltf::GltfMesh, prelude::*};
use leafwing_input_manager::prelude::{ActionState, InputMap, VirtualDPad};
use lightyear::prelude::{
    client::{Confirmed, Interpolated, Predicted},
    server::ReplicationTarget,
};
use pha_assets::{LevelState, assets::GlobalAssets};
use pha_protocol::{
    component::{Player, ViewDirection},
    input::NetworkedInput,
};

use crate::{Rendered, Simulated};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_player_gameplay_components).run_if(in_state(LevelState::Loaded)),
        );

        app.add_systems(FixedUpdate, rotate_player_from_view_direction);

        // app.add_systems(FixedUpdate, (move_player, rotate_player_from_view));
    }
}

fn add_player_gameplay_components(
    mut commands: Commands,
    q_rendered_player: Query<Entity, (Rendered, Without<RigidBody>, With<Player>)>,
    global_assets: Res<GlobalAssets>,
) {
    if q_rendered_player.is_empty() {
        return;
    }

    for player_entity in &q_rendered_player {
        commands.entity(player_entity).insert((
            RigidBody::Kinematic,
            Collider::capsule(3.0, 4.0),
            SceneRoot(global_assets.character.clone()),
        ));
    }
}

const PLAYER_MOVE_SPEED: f32 = 3000.0;

pub fn move_player(
    mut q_player: Query<
        (&ActionState<NetworkedInput>, &mut LinearVelocity),
        (Simulated, With<Player>),
    >,
    time: Res<Time>,
) {
    for (action_state, mut velocity) in q_player.iter_mut() {
        if let Some(movement) = action_state.dual_axis_data(&NetworkedInput::Move) {
            let move_vec = Vec3::new(movement.pair.x, 0.0, -movement.pair.y).normalize();
            velocity.0 = move_vec * PLAYER_MOVE_SPEED * time.delta_secs();
        }
    }
}

// Rotates the player's transform based on their ViewDirection component
//

pub fn rotate_player_from_view_direction(
    mut q_player: Query<(&mut Rotation, &Children), With<Player>>,
    q_view_direction: Query<(Entity, &ViewDirection)>,
) {
    for (mut rotation, children) in q_player.iter_mut() {
        // find the ViewDirection child
        for child in children.iter() {
            if let Ok(view_direction) = q_view_direction.get(*child) {
                println!("applied rotation server side");
                // Get the quaternion from ViewDirection and convert to Bevy's Quat if needed
                let view_quat: Quat = view_direction.1.0;
                // Only apply yaw rotation to keep the player model upright
                let (_, yaw, _) = view_quat.to_euler(EulerRot::YXZ);
                rotation.0 = Quat::from_rotation_y(yaw);
            }
        }
    }
}
