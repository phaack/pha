use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands
                .spawn(Camera3d::default())
                .insert(Transform::from_xyz(-15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y));
        });
    }
}
