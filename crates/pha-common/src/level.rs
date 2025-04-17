use avian3d::prelude::{Collider, RigidBody};
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for geo in &q_geo {
        commands.entity(geo).insert(RigidBody::Static);
    }
    // commands.spawn((
    //     Transform::from_xyz(0., -5., 0.),
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(128.0, 128.0))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(0, 200, 0))),
    //     RigidBody::Static,
    //     Collider::half_space(Vec3::Y),
    // ));
    //
    // // Spawn a little platform for the player to jump on.
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(4.0, 1.0, 4.0))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(240, 0, 0))),
    //     Transform::from_xyz(-6.0, 2.0, 0.0),
    //     RigidBody::Static,
    //     Collider::cuboid(4.0, 1.0, 4.0),
    // ));
}
