use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, Default, Reflect, PartialEq)]
pub struct NetworkedCameraTransform(pub Transform);
