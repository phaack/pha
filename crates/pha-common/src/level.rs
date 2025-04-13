use avian3d::prelude::{AngularVelocity, Collider, RigidBody};
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
            commands.spawn(SceneRoot(level_assets.example_level.clone()));
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
    for geo in &q_geo {
        commands.entity(geo).insert(RigidBody::Static);
    }

    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(15.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(15.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // insert a cube
    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(2.0, 2.0, 2.0),
        AngularVelocity(Vec3::new(2.5, 0.0, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(2.0, 10.0, 2.0),
    ));
}
