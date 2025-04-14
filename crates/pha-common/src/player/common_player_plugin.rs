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

pub struct CommonPlayerPlugin;

impl Plugin for CommonPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_player_gameplay_components).run_if(in_state(LevelState::Loaded)),
        );
    }
}

// add Collider to player on spawn
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
