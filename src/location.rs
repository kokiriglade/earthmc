//! # Location
//!
//! Defines the [`Location`] struct.
use std::fmt::{self, Formatter};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeTuple,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Location {
    pub world: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
}
