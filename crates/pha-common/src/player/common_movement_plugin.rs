use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use pha_protocol::{component::Player, input::NetworkedInput};

use crate::Simulated;

const PLAYER_MOVE_SPEED: f32 = 3000.0;

pub(crate) struct CommonMovemenPlugin;

impl Plugin for CommonMovemenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_player);
    }
}

fn move_player(
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
