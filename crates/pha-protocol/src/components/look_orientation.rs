use avian3d::math::Quaternion;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
pub struct LookOrientation(pub Quaternion);

impl LookOrientation {
    /// Returns the forward direction vector based on the LookOrientation.
    pub fn forward(&self) -> Vec3 {
        self.0.mul_vec3(Vec3::Z)
    }

    /// Returns the backward direction vector.
    pub fn backward(&self) -> Vec3 {
        -self.forward()
    }

    /// Returns the right direction vector.
    pub fn right(&self) -> Vec3 {
        self.0.mul_vec3(Vec3::X)
    }

    /// Returns the left direction vector.
    pub fn left(&self) -> Vec3 {
        -self.right()
    }

    /// Returns the up direction vector.
    pub fn up(&self) -> Vec3 {
        self.0.mul_vec3(Vec3::Y)
    }

    /// Returns the down direction vector.
    pub fn down(&self) -> Vec3 {
        -self.up()
    }

    /// Converts the LookOrientation to Yaw (rotation around the Y-axis) and Pitch (rotation around the X-axis).
    /// Returns (yaw_radians, pitch_radians).
    pub fn to_yaw_pitch(&self) -> (f32, f32) {
        let forward = self.forward().normalize();
        let pitch = forward.y.asin();
        let yaw = forward.x.atan2(forward.z); // Corrected atan2 usage for Vec3 components
        (yaw, pitch)
    }

    /// Creates a LookOrientation from Yaw (rotation around Y) and Pitch (rotation around X) in radians.
    pub fn from_yaw_pitch(yaw: f32, pitch: f32) -> Self {
        Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0).into() // Corrected from_euler usage with EulerRot
    }

    /// Creates a LookOrientation that looks in the given direction.
    /// This assumes the "forward" of your model/world is +Z.
    pub fn look_at(direction: Vec3) -> Self {
        let target = direction.normalize();
        let forward = Vec3::Z;
        let rotation = Quat::from_rotation_arc(forward, target);
        LookOrientation(rotation)
    }

    /// Returns the underlying Bevy Quat.
    pub fn as_quat(&self) -> Quat {
        self.0
    }
}

impl From<Quat> for LookOrientation {
    fn from(quat: Quat) -> Self {
        LookOrientation(quat)
    }
}

impl From<LookOrientation> for Quat {
    fn from(look_orientation: LookOrientation) -> Self {
        look_orientation.0
    }
}
