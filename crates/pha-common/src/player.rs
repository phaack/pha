use avian3d::prelude::{Collider, LinearVelocity, LockedAxes, RigidBody, collider};
use bevy::{gltf::GltfMesh, prelude::*};
use bevy_tnua::{
    TnuaUserControlsSystemSet,
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::{ActionState, InputMap, VirtualDPad};
use lightyear::prelude::{
    client::{Confirmed, Interpolated, Predicted},
    server::ReplicationTarget,
};
use pha_assets::{LevelState, assets::GlobalAssets};
use pha_protocol::{component::Player, input::NetworkedInput};

use crate::{Rendered, Simulated};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_player_gameplay_components).run_if(in_state(LevelState::Loaded)),
        );
        app.add_systems(
            FixedUpdate,
            apply_controls_with_tnua.in_set(TnuaUserControlsSystemSet),
        );

        // app.add_systems(FixedUpdate, move_player);
    }
}

fn add_player_gameplay_components(
    mut commands: Commands,
    q_rendered_player: Query<Entity, (Rendered, Without<RigidBody>, With<Player>)>,
    global_assets: Res<GlobalAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if q_rendered_player.is_empty() {
        return;
    }

    for player_entity in &q_rendered_player {
        commands.entity(player_entity).insert((
            RigidBody::Dynamic,
            Mesh3d(meshes.add(Capsule3d {
                radius: 0.5,
                half_length: 0.5,
            })),
            MeshMaterial3d(materials.add(Color::srgb_u8(50, 50, 50))),
            Transform::from_xyz(0.0, 2.0, 0.0),
            Collider::capsule(0.5, 1.0),
            // Collider::capsule(3.0, 4.0),
            // SceneRoot(global_assets.character.clone()),
            // This is Tnua's interface component.
            TnuaController::default(),
            // A sensor shape is not strictly necessary, but without it we'll get weird results.
            TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
            // Tnua can fix the rotation, but the character will still get rotated before it can do so.
            // By locking the rotation we can prevent this.
            LockedAxes::ROTATION_LOCKED,
        ));
    }
}

// const PLAYER_MOVE_SPEED: f32 = 30.0;
//
// fn move_player(
//     mut q_player: Query<
//         (&ActionState<NetworkedInput>, &mut LinearVelocity),
//         (Simulated, With<Player>),
//     >,
// ) {
//     for (action_state, mut velocity) in q_player.iter_mut() {
//         if let Some(movement) = action_state.dual_axis_data(&NetworkedInput::Move) {
//             let move_vec = Vec3::new(movement.pair.x, 0.0, -movement.pair.y).normalize();
//             velocity.0 = move_vec * PLAYER_MOVE_SPEED;
//         }
//     }
// }

// 2. Constants for movement parameters
const PLAYER_MOVE_SPEED: f32 = 10.0; // Adjust as needed (Tnua interprets this differently than direct velocity)
const PLAYER_JUMP_HEIGHT: f32 = 4.0; // Adjust as needed
// IMPORTANT: This MUST be set correctly for your character collider.
// It should be slightly larger than the distance from the character's
// center of mass to the lowest point of its collider.
const PLAYER_FLOAT_HEIGHT: f32 = 1.5;

// --- The Bevy System ---

fn apply_controls_with_tnua(
    // Query for player entities with both input state and Tnua controller
    mut q_player: Query<
        (&ActionState<NetworkedInput>, &mut TnuaController),
        (Simulated, With<Player>),
    >,
) {
    for (action_state, mut controller) in q_player.iter_mut() {
        // --- Handle Basis (Walking/Movement) ---

        // Start with a default walk basis configuration.
        // We modify desired_velocity based on input.
        let mut walk_basis = TnuaBuiltinWalk {
            // Set the float height specific to your character collider.
            float_height: PLAYER_FLOAT_HEIGHT,
            // Configure other walk parameters if needed (e.g., acceleration, turning speed)
            // Otherwise, defaults are used.
            ..Default::default()
        };

        if let Some(movement) = action_state.dual_axis_data(&NetworkedInput::Move) {
            // Get the Vec2 axis pair from the input data
            let move_axis_pair = movement.pair;

            // Calculate the desired movement direction in 3D space.
            // Map the Y input axis to the negative Z world axis.
            let desired_direction =
                Vec3::new(move_axis_pair.x, 0.0, -move_axis_pair.y).normalize_or_zero(); // Ensure it's a unit vector (or zero if input is zero)

            // Set the desired velocity for the walk basis.
            // Tnua will use this to determine how to accelerate the character.
            walk_basis.desired_velocity = desired_direction * PLAYER_MOVE_SPEED;
        } else {
            // If there's no movement input, the desired velocity is zero.
            // This tells Tnua the character should try to stop.
            walk_basis.desired_velocity = Vec3::ZERO;
        }

        // Feed the walk basis to the Tnua controller *every frame*.
        // This is necessary for Tnua to continuously manage movement and grounding.
        controller.basis(walk_basis);

        // --- Handle Actions (Jumping) ---

        // Check if the Jump action is currently pressed.
        // Use action_state.pressed(...) if you want the action fed as long as held.
        // Use action_state.just_pressed(...) if you only want it fed on the frame it starts.
        // The Tnua example uses `pressed`, so we'll stick with that for jump height control.
        if action_state.just_pressed(&NetworkedInput::Jump) {
            // Feed the jump action to the controller.
            controller.action(TnuaBuiltinJump {
                height: PLAYER_JUMP_HEIGHT,
                // Configure other jump parameters if needed.
                ..Default::default()
            });
        }
        // Note: If you stop feeding the jump action (e.g., the button is released),
        // Tnua might cut the jump short depending on its configuration.
    }
}
