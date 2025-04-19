use avian3d::prelude::{Collider, Position, RigidBody};
use bevy::prelude::*;
use lightyear::prelude::*;
use pha_assets::{CurrentLevel, Geometry, LevelState, assets::LevelAssets};
use pha_protocol::message::Level;

pub struct LevelPlugin;

// A "Level" represents every non-replicated object in your environment.
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelState::Loaded),
            (level_loaded, add_level_gameplay_components).chain(),
        );
    }
}

fn level_loaded(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    level_assets: Res<LevelAssets>,
) {
    match **current_level {
        Level::Example => {
            // commands.spawn(SceneRoot(level_assets.example_level.clone()));
        }
        Level::Void => {}
    }
}

fn add_level_gameplay_components(
    mut commands: Commands,
    q_geo: Query<Entity, (With<Geometry>, Without<RigidBody>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // println!("Adding map RigidBody static");
    // for geo in &q_geo {
    //     commands.entity(geo).insert(RigidBody::Static);
    // }

    // let ground_size = Vec3::new(20.0, 0.5, 20.0);
    // let wall_size = Vec3::new(0.1, 2.0, 2.0);
    // let box_size = Vec3::new(1.0, 1.0, 1.0);
    // let step_depth = 0.5;
    // let step_width = 1.0;
    //
    // // example
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::from_size(ground_size))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb_u8(200, 0, 0),
    //         ..Default::default()
    //     })),
    //     Collider::cuboid(
    //         ground_size.x / 2.0,
    //         ground_size.y / 2.0,
    //         ground_size.z / 2.0,
    //     ),
    //     RigidBody::Static,
    //     Geometry,
    // ));

    // --- Ground ---
    let ground_size = Vec3::new(20.0, 0.5, 20.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(ground_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(100, 100, 100), // Gray
            ..Default::default()
        })),
        Collider::cuboid(ground_size.x, ground_size.y, ground_size.z),
        RigidBody::Static,
        Geometry,
        Name::new("Ground"),
    ));

    // --- Wall ---
    let wall_size = Vec3::new(0.1, 2.0, 2.0);
    commands.spawn((
        Transform::from_xyz(5.0, wall_size.y / 2.0, 0.0),
        Mesh3d(meshes.add(Cuboid::from_size(wall_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(150, 50, 50), // Red-ish
            ..Default::default()
        })),
        Collider::cuboid(wall_size.x, wall_size.y, wall_size.z),
        RigidBody::Static,
        Geometry,
        Name::new("Wall"),
    ));

    // --- Jump Box ---
    let box_size = Vec3::new(1.0, 1.0, 1.0);
    commands.spawn((
        Transform::from_xyz(-5.0, box_size.y / 2.0, 3.0),
        Mesh3d(meshes.add(Cuboid::from_size(box_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(50, 150, 50), // Green-ish
            ..Default::default()
        })),
        Collider::cuboid(box_size.x / 2.0, box_size.y / 2.0, box_size.z / 2.0),
        RigidBody::Static,
        Geometry,
        Name::new("JumpBox"),
    ));

    // --- Stairs Set 1: 5 Steps ---
    let num_steps1 = 5;
    let step_height1 = 1.0 / num_steps1 as f32;
    let step_depth = 0.5;
    let step_width = 1.0;
    let stair1_start_x = -7.0;

    for i in 0..num_steps1 {
        let z_position = step_height1 * (i as f32 + 0.5);
        let x_position = stair1_start_x + i as f32 * (step_depth + 0.1);
        commands.spawn((
            Transform::from_xyz(x_position, z_position, -3.0),
            Mesh3d(meshes.add(Cuboid::new(step_width, step_depth, step_height1))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb_u8(120, 80, 40), // Brown-ish
                ..Default::default()
            })),
            Collider::cuboid(step_width, step_depth, step_height1),
            RigidBody::Static,
            Geometry,
            Name::new(format!("Stair1_{}", i + 1)),
        ));
    }

    // --- Stairs Set 2: 3 Steps ---
    let num_steps2 = 3;
    let step_height2 = 1.0 / num_steps2 as f32;
    let stair2_start_x = stair1_start_x + (num_steps1 as f32 + 1.0) * (step_depth + 0.1);

    for i in 0..num_steps2 {
        let z_position = step_height2 * (i as f32 + 0.5);
        let x_position = stair2_start_x + i as f32 * (step_depth + 0.1);
        commands.spawn((
            Transform::from_xyz(x_position, z_position, -3.0),
            Mesh3d(meshes.add(Cuboid::new(step_width, step_depth, step_height2))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb_u8(120, 80, 40), // Brown-ish
                ..Default::default()
            })),
            Collider::cuboid(step_width, step_depth, step_height2),
            RigidBody::Static,
            Geometry,
            Name::new(format!("Stair2_{}", i + 1)),
        ));
    }

    // --- Stairs Set 3: 7 Steps ---
    let num_steps3 = 7;
    let step_height3 = 1.0 / num_steps3 as f32;
    let stair3_start_x = stair2_start_x + (num_steps2 as f32 + 1.0) * (step_depth + 0.1);

    for i in 0..num_steps3 {
        let z_position = step_height3 * (i as f32 + 0.5);
        let x_position = stair3_start_x + i as f32 * (step_depth + 0.1);
        commands.spawn((
            Transform::from_xyz(x_position, z_position, -3.0),
            Mesh3d(meshes.add(Cuboid::new(step_width, step_depth, step_height3))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb_u8(120, 80, 40), // Brown-ish
                ..Default::default()
            })),
            Collider::cuboid(step_width, step_depth, step_height3),
            RigidBody::Static,
            Geometry,
            Name::new(format!("Stair3_{}", i + 1)),
        ));
    }

    // --- Upward Slope ---
    let slope_length = 3.0;
    let slope_width = 3.0;
    let slope_height = 1.0; // Target height difference
    let num_segments = 10; // Approximate the slope with multiple cuboids
    for i in 0..num_segments {
        let start_z = 0.0;
        let end_z = slope_height;
        let current_z = start_z + (end_z - start_z) * (i as f32 / num_segments as f32);
        let next_z = start_z + (end_z - start_z) * ((i + 1) as f32 / num_segments as f32);
        let segment_height = (next_z - current_z).abs();
        let segment_center_z = (current_z + next_z) / 2.0;
        let segment_x = 2.0 + slope_length * (i as f32 + 0.5) / num_segments as f32;
        let segment_y = -3.0;
        let segment_size = Vec3::new(
            slope_length / num_segments as f32,
            slope_width,
            segment_height,
        );

        commands.spawn((
            Transform::from_xyz(segment_x, segment_center_z, segment_y),
            Mesh3d(meshes.add(Cuboid::from_size(segment_size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb_u8(200, 200, 0), // Yellow-ish
                ..Default::default()
            })),
            Collider::cuboid(segment_size.x, segment_size.y, segment_size.z),
            RigidBody::Static,
            Geometry,
            Name::new(format!("UpwardSlopeSegment_{}", i + 1)),
        ));
    }

    // --- Downward Slope ---
    for i in 0..num_segments {
        let start_z = slope_height;
        let end_z = 0.0;
        let current_z = start_z + (end_z - start_z) * (i as f32 / num_segments as f32);
        let next_z = start_z + (end_z - start_z) * ((i + 1) as f32 / num_segments as f32);
        let segment_height = (next_z - current_z).abs();
        let segment_center_z = (current_z + next_z) / 2.0;
        let segment_x = 7.0 + slope_length * (i as f32 + 0.5) / num_segments as f32;
        let segment_y = -3.0;
        let segment_size = Vec3::new(
            slope_length / num_segments as f32,
            slope_width,
            segment_height,
        );

        commands.spawn((
            Transform::from_xyz(segment_x, segment_center_z, segment_y),
            Mesh3d(meshes.add(Cuboid::from_size(segment_size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb_u8(200, 200, 0), // Yellow-ish
                ..Default::default()
            })),
            Collider::cuboid(segment_size.x, segment_size.y, segment_size.z),
            RigidBody::Static,
            Geometry,
            Name::new(format!("DownwardSlopeSegment_{}", i + 1)),
        ));
    }
}
