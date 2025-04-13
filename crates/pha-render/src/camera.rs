use bevy::prelude::*;

// marker for default cam (mainMenu etc.)
#[derive(Component)]
pub struct DefaultCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands
                .spawn(Camera3d::default())
                .insert(Transform::from_xyz(-50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y))
                .insert(DefaultCamera);
        });
    }
}
