//! # World Location
//!
//! Defines the [`WorldLocation`] struct.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WorldLocation {
    pub world: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
}
