//! # Discord Link
//!
//! Defines the [`DiscordLink`] struct.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DiscordLink {
    /// Discord User ID.
    pub id: String,
    /// Minecraft UUID.
    pub uuid: Uuid,
}
