//! # Quarter
//!
//! Defines the [`Quarter`] struct.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    block_position::BlockPosition,
    named_id::{NamedId, NamedIdOpt},
};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quarter {
    /// The Quarter's name.
    pub name: String,
    /// The Quarter's [`Uuid`].
    pub uuid: Uuid,
    /// The Quarter's kind, or as it is referred to in the API, its `type`.
    #[serde(rename = "type")]
    pub kind: QuarterKind,
    /// The [`Uuid`] of the creator of the Quarter.
    pub creator: Uuid,
    /// The Quarter's owner.
    pub owner: NamedIdOpt,
    /// The town this Quarter is inside.
    pub town: NamedId,
    pub timestamps: QuarterTimestamps,
    pub status: QuarterStatus,
    pub stats: QuarterStats,
    /// Tuple of RGBA values of the Quarter.
    pub colour: [u8; 4],
    /// All of the trusted players in this Quarter.
    pub trusted: Vec<NamedId>,
    /// Every cuboid in this Quarter.
    pub cuboids: Vec<QuarterCuboid>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuarterKind {
    /// The default kind of Quarter.
    Apartment,
    /// Allows bed usage.
    Inn,
    /// Allows vehicle placing and usage.
    Station,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuarterTimestamps {
    /// Unix timestamp representing the time the Quarter was created.
    pub registered: i64,
    /// Unix timestamp representing the time the Quarter was claimed.
    pub claimed_at: Option<i64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuarterStatus {
    /// If the Quarter is an embassy.
    pub is_embassy: bool,
    /// If the Quarter is for sale.
    pub is_for_sale: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuarterStats {
    /// The Quarter's sale price.
    pub price: Option<i32>,
    /// The total number of blocks within the Quarter's bounds.
    pub volume: i32,
    /// The total amount of cuboids this Quarter is made of.
    pub num_cuboids: u16,
    /// The size of the particles displayed in-game.
    pub particle_size: Option<f32>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuarterCuboid {
    pub corner_one: BlockPosition,
    pub corner_two: BlockPosition,
}
