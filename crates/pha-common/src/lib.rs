use avian3d::{PhysicsPlugins, prelude::PhysicsInterpolationPlugin};
use bevy::prelude::*;
use lightyear::prelude::{
    PreSpawnedPlayerObject, ReplicationGroup,
    client::{Interpolated, Predicted, VisualInterpolateStatus},
    server::ReplicationTarget,
};
use pha_assets::AssetPlugin;
use pha_protocol::ProtocolPlugin;

pub mod level;
pub mod player;

use player::common_player_plugin::CommonPlayerPlugin;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetPlugin,
            ProtocolPlugin,
            PhysicsPlugins::new(FixedPostUpdate)
                .build()
                .disable::<PhysicsInterpolationPlugin>(),
            level::LevelPlugin,
            //
            CommonPlayerPlugin,
        ));
    }
}

pub const REPLICATION_GROUP_PREDICTED: ReplicationGroup = ReplicationGroup::new_id(42);

pub type Simulated = Or<(
    With<Predicted>,
    With<PreSpawnedPlayerObject>,
    With<ReplicationTarget>,
)>;
pub type Rendered = Or<(Simulated, With<Interpolated>)>;
