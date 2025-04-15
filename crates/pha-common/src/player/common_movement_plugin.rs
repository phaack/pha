use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::{prelude::TickManager, shared::tick_manager};
use pha_protocol::{component::Player, input::NetworkedInput};

use crate::Simulated;

const PLAYER_MOVE_SPEED: f32 = 30.0;

pub(crate) struct CommonMovemenPlugin;

impl Plugin for CommonMovemenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_player);
    }
}

fn move_player(
    mut q_player: Query<
        (Entity, &ActionState<NetworkedInput>, &mut LinearVelocity),
        (Simulated, With<Player>),
    >,
    // This query gets look orientation components specifically for player entities
    q_player_look: Query<(&Parent)>,
    time: Res<Time>,
    tick_manager: Res<TickManager>,
) {
    // for (player_entity, action_state, mut velocity) in q_player.iter_mut() {
    //     if let Some(movement) = action_state.dual_axis_data(&NetworkedInput::Move) {
    //         if movement.pair.length() == 0. {
    //             // If no movement input, stop horizontal movement
    //             velocity.0.x = 0.0;
    //             velocity.0.z = 0.0;
    //             continue;
    //         }
    //         // Get all look orientation components with player_entity as parent
    //         for (parent) in &q_player_look {
    //             if parent.get() == player_entity {
    //                 // Get input direction vector in local space (relative to player's local axes)
    //                 let local_move_vec =
    //                     Vec3::new(movement.pair.x, 0.0, movement.pair.y).normalize_or_zero();
    //
    //                 // The LookOrientation represents the direction the player is facing.
    //                 // We want to move *forward* relative to that orientation on the XZ plane.
    //
    //                 // 1. Extract the forward vector from the LookOrientation.
    //                 let forward_direction = look_orientation.forward();
    //
    //                 // 2. Project the forward direction onto the XZ plane (set Y to 0).
    //                 let forward_on_xz = Vec3::new(forward_direction.x, 0.0, forward_direction.z)
    //                     .normalize_or_zero();
    //
    //                 // 3. Calculate the right vector based on the player's orientation (for strafing).
    //                 let right_direction = look_orientation.right();
    //                 let right_on_xz =
    //                     Vec3::new(right_direction.x, 0.0, right_direction.z).normalize_or_zero();
    //
    //                 // 4. Combine the forward and right movements based on the input.
    //                 let mut world_move_vec = Vec3::ZERO;
    //                 world_move_vec += forward_on_xz * local_move_vec.z; // Forward/Backward is along local +Z
    //                 world_move_vec += right_on_xz * -local_move_vec.x; // Left/Right is along local +X
    //                 world_move_vec = world_move_vec.normalize_or_zero();
    //
    //                 // Keep the Y velocity at 0 to prevent flying
    //                 velocity.0.y = 0.0;
    //
    //                 // Apply the movement scaled by speed and time
    //                 velocity.0 += world_move_vec * PLAYER_MOVE_SPEED * time.delta_secs();
    //
    //                 println!(
    //                     "on tick {:?} using LookOrientation: {:?} and input: {}",
    //                     tick_manager.tick(),
    //                     look_orientation,
    //                     movement.pair
    //                 );
    //
    //                 break; // Found the right component, no need to check others
    //             }
    //         }
    //     }
    // }
}
