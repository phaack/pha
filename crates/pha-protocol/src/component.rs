use avian3d::{math::Quaternion, prelude::*};
use bevy::prelude::*;
use lightyear::{
    prelude::{
        client::{ComponentSyncMode, LerpFn},
        *,
    },
    utils::bevy::TransformLinearInterpolation,
};

use crate::components::look_orientation::LookOrientation;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player(pub ClientId);

pub fn register_components(app: &mut App) {
    app.register_type::<LookOrientation>();
    app.register_component::<LookOrientation>(ChannelDirection::Bidirectional)
        // .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(|start, end, t| Quaternion::slerp(start.0, end.0, t).into());

    app.register_component::<Player>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once)
        .add_interpolation(ComponentSyncMode::Once);

    app.register_component::<Position>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(|start, end, t| Position(start.lerp(**end, t)))
        .add_correction_fn(|start, end, t| Position(start.lerp(**end, t)));

    app.add_interpolation_fn::<Transform>(TransformLinearInterpolation::lerp);
}
