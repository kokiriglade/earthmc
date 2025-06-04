//! # Location
//!
//! Defines the [`LocationInfo`] struct.
use serde::{Deserialize, Serialize};

use crate::named_id::NamedIdOpt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocationInfo {
    pub location: PositionXZ,
    pub is_wilderness: bool,
    pub town: NamedIdOpt,
    pub nation: NamedIdOpt,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PositionXZ {
    pub x: f32,
    pub z: f32,
}
