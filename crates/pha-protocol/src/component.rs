use avian3d::prelude::*;
use bevy::{math::VectorSpace, prelude::*};
use lightyear::{
    prelude::{
        client::{ComponentSyncMode, LerpFn},
        *,
    },
    utils::bevy::TransformLinearInterpolation,
};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Grounded;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player(pub ClientId);

pub fn register_components(app: &mut App) {
    app.register_component::<Player>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once)
        .add_interpolation(ComponentSyncMode::Once);

    app.register_component::<Position>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(|start, end, t| Position(start.lerp(**end, t)))
        .add_correction_fn(|start, end, t| Position(start.lerp(**end, t)));

    app.register_component::<Rotation>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(|start, end, t| Rotation(*start.slerp(*end, t)))
        .add_correction_fn(|start, end, t| Rotation(*start.slerp(*end, t)));

    app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(|start, end, t| LinearVelocity(start.0.lerp(end.0, t)));

    app.register_component::<Grounded>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.add_interpolation_fn::<Transform>(TransformLinearInterpolation::lerp);
}
