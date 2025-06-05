//! # Mystery Master
//!
//! Defines the [`MysteryMaster`] struct.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a player participating in "Mystery Master". Change indiciates in
/// what direction they have moved on the scoreboard.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MysteryMaster {
    /// The player's username.
    pub name: String,
    /// The player's [`Uuid`].
    pub uuid: Uuid,
    /// The change kind.
    pub change: MysteryMasterChangeKind,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MysteryMasterChangeKind {
    Down,
    Unchanged,
    Up,
}
