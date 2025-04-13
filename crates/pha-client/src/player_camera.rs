use bevy::prelude::*;
use pha_render::camera::DefaultCamera;

use crate::{game_state::GameState, replication::LocalPlayer};

pub(crate) struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), enable_default_camera);
        app.add_systems(OnEnter(GameState::MainMenu), enable_default_camera);
        app.add_systems(
            OnEnter(GameState::Playing),
            (disable_default_camera, add_player_camera),
        );
    }
}

fn add_player_camera(mut q_player: Query<Entity, With<LocalPlayer>>, mut commands: Commands) {
    let player_camera = commands.spawn(Camera3dBundle::default()).id();
    for entity in q_player.iter() {
        commands.entity(entity).add_child(player_camera);
        println!("player camera has been spawned");
    }
}

// camera management
fn disable_default_camera(mut q_camera: Query<&mut Camera, With<DefaultCamera>>) {
    let cam = q_camera.get_single_mut().ok();

    if let Some(mut c) = cam {
        c.is_active = false;
    }
}

fn enable_default_camera(mut q_camera: Query<&mut Camera, With<DefaultCamera>>) {
    let cam = q_camera.get_single_mut().ok();

    if let Some(mut c) = cam {
        c.is_active = true;
    }
}
