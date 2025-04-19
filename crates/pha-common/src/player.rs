use avian3d::{
    math::Quaternion,
    prelude::{
        Collider, CollisionMargin, Gravity, GravityScale, LinearVelocity, Mass, PhysicsGizmoExt,
        PhysicsGizmos, PhysicsSet, Position, RigidBody, Rotation, ShapeCastConfig, ShapeHitData,
        SpatialQuery, SpatialQueryFilter, collider,
    },
    spatial_query,
};
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputMap, VirtualDPad};
use lightyear::prelude::{
    client::{Confirmed, Interpolated, Predicted},
    server::ReplicationTarget,
};
use pha_assets::{LevelState, assets::GlobalAssets};
use pha_protocol::{
    component::{Grounded, Player},
    input::NetworkedInput,
};

use crate::{Rendered, Simulated};

//------------------------------------------------------------------------------
// PLUGIN REGISTRATION
//------------------------------------------------------------------------------

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_player_gameplay_components).run_if(in_state(LevelState::Loaded)),
        );

        app.add_systems(
            FixedPostUpdate,
            (update_grounded, apply_movement, apply_damping)
                .chain()
                .before(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
        app.insert_resource(Gravity(Vec3::NEG_Y * 9.6));
    }
}

//------------------------------------------------------------------------------
// COMPONENT DEFINITIONS
//------------------------------------------------------------------------------

/// The main component for character control, containing movement parameters
#[derive(Component)]
pub struct KinematicCharacterController {
    pub max_speed: f32,
    pub acceleration: f32,
    pub jump_impulse: f32,
    pub skin_width: f32,
}

impl Default for KinematicCharacterController {
    fn default() -> Self {
        Self {
            max_speed: 7.0,
            acceleration: 5.0,
            jump_impulse: 2.0,
            skin_width: 0.1,
        }
    }
}

//------------------------------------------------------------------------------
// INITIALIZATION SYSTEMS
//------------------------------------------------------------------------------

fn add_player_gameplay_components(
    mut commands: Commands,
    q_rendered_player: Query<Entity, (Rendered, Without<RigidBody>, With<Player>)>,
    global_assets: Res<GlobalAssets>,
) {
    if q_rendered_player.is_empty() {
        return;
    }

    for player_entity in &q_rendered_player {
        println!(
            "Initializing player entity {:?} with RigidBody::Kinematic",
            player_entity
        );

        // Create a capsule collider
        let radius = 0.3;
        let height = 1.;
        let collider = Collider::capsule(radius - 0.05, height);

        // Add components
        commands.entity(player_entity).insert((
            RigidBody::Kinematic,
            Name::new("Player"),
            collider,
            CollisionMargin(0.05),
            KinematicCharacterController::default(),
            LinearVelocity(Vec3::ZERO),
        ));
    }
}

//------------------------------------------------------------------------------
// MOVEMENT SYSTEMS
//------------------------------------------------------------------------------

/// Update the grounded state for character controllers
fn update_grounded(
    mut commands: Commands,
    query: Query<
        (Entity, &Position, &Collider, Option<&Grounded>),
        With<KinematicCharacterController>,
    >,
    spatial_query: SpatialQuery,
) {
    for (entity, position, collider, grounded) in &query {
        let is_grounded = is_on_floor(
            position.0,
            collider,
            &spatial_query,
            &SpatialQueryFilter::from_excluded_entities([entity]),
        );

        // Add or remove Grounded component based on detection
        if is_grounded && grounded.is_none() {
            commands.entity(entity).insert(Grounded);
        } else if !is_grounded && grounded.is_some() {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Simplified movement system with basic collision detection
fn apply_movement(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &KinematicCharacterController,
            &mut LinearVelocity,
            &Position,
            &Rotation,
            &Collider,
            &ActionState<NetworkedInput>,
            Option<&Grounded>,
        ),
        Simulated,
    >,
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut gizmos: Gizmos<PhysicsGizmos>,
) {
    for (entity, controller, mut velocity, position, rotation, collider, input, grounded) in
        query.iter_mut()
    {
        // Get input direction
        let mut input_direction = Vec3::ZERO;
        if let Some(movement) = input.dual_axis_data(&NetworkedInput::Move) {
            input_direction = rotation
                .mul_vec3(Vec3::new(movement.pair.x, 0.0, -movement.pair.y))
                .normalize_or_zero();
        }

        // Filter to exclude the player entity itself
        let filter = &SpatialQueryFilter::from_excluded_entities([entity]);

        let final_movement = input_direction * time.delta_secs();

        let new_motion = collide_and_slide(
            position.0,
            input_direction,
            collider,
            &spatial_query,
            filter,
            false,
            1,
            &mut gizmos,
        );

        // Apply the final movement
        if final_movement.length_squared() > 0.00001 {
            commands
                .entity(entity)
                .insert(Position(position.0 + final_movement));
        }
    }
}

/// returns the new point where we should land
/// therefore direction = new_pos - old_pos
fn collide_and_slide(
    pos: Vec3,
    vel: Vec3,
    collider: &Collider,
    spatial_query: &SpatialQuery,
    filter: &SpatialQueryFilter,
    vertical_run: bool,
    depth: u8,
    mut gizmos: &mut Gizmos<PhysicsGizmos>,
) -> Option<Vec3> {
    if let Ok(dir) = Dir3::from_xyz(vel.x, vel.y, vel.z) {
        if let Some(hit) = spatial_query.cast_shape(
            collider,
            pos,
            Quat::IDENTITY,
            dir,
            &ShapeCastConfig {
                max_distance: vel.length(),
                ..default()
            },
            filter,
        ) {
            println!("Found hit {} away", hit.distance);
            let vec_to_hit = (pos - hit.point1).normalize() * hit.distance;
            // project vec_to_hit onto the plane with normal hit.normal1
            let projected = project_onto_plane(vec_to_hit, hit.normal1);
            println!("Projected: {:?}", projected);
            gizmos.draw_line(pos, pos + projected, Color::srgb_u8(0, 250, 100));

            return None;
        } else {
            return Some(pos);
        }
    }
    None
}

fn project_onto_plane(vec: Vec3, normal: Vec3) -> Vec3 {
    vec - normal * vec.dot(normal)
}

/// Apply velocity damping to simulate friction
fn apply_damping(
    mut query: Query<(&mut LinearVelocity, Option<&Grounded>), With<KinematicCharacterController>>,
) {
    for (mut velocity, grounded) in &mut query {
        // Apply stronger damping when grounded
        let damp_factor = if grounded.is_some() { 0.8 } else { 0.95 };

        // Only damp horizontal movement
        velocity.0.x *= damp_factor;
        velocity.0.z *= damp_factor;

        // Apply small threshold to stop very slow movement completely
        if velocity.0.x.abs() < 0.01 {
            velocity.0.x = 0.0;
        }
        if velocity.0.z.abs() < 0.01 {
            velocity.0.z = 0.0;
        }
    }
}

//------------------------------------------------------------------------------
// COLLISION DETECTION
//------------------------------------------------------------------------------

/// Determine if the character is on a walkable floor surface
pub fn is_on_floor(
    origin: Vec3,
    collider: &Collider,
    spatial_query: &SpatialQuery,
    filter: &SpatialQueryFilter,
) -> bool {
    if let Some(_) = spatial_query.cast_shape(
        collider,
        origin,
        Quat::IDENTITY,
        Dir3::NEG_Y,
        &ShapeCastConfig {
            max_distance: 0.2,
            ..Default::default()
        },
        &filter.clone(),
    ) {
        true
    } else {
        false
    }
}
