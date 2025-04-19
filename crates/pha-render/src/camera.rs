use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera3d::default()).insert(
                Transform::from_xyz(4., 6.5, 9.0)
                    .looking_at(Vec3::from_array([3., 2., 4.0]), Vec3::Y),
            );
        });
    }
}
