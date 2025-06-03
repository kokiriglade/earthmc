//! # Shared
//!
//! Defines common structs found throughout the API, such as [`NamedId`].
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An object with a name and a UUID.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NamedId {
    /// The thing's name.
    pub name: String,
    /// The thing's UUID.
    pub uuid: Uuid,
}
