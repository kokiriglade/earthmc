//! # Discord Link
//!
//! Defines the [`DiscordLink`] struct.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DiscordLink {
    /// Discord User ID.
    id: String,
    /// Minecraft UUID.
    uuid: Uuid,
}
