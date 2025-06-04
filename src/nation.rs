//! # Nation
//!
//! Defines the [`Nation`] struct.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{location::Location, named_id::NamedId};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Nation {
    /// The nation's name.
    pub name: String,
    /// The nation's [`Uuid`].
    pub uuid: Uuid,
    /// The nation's board as seen on `/n` in-game.
    pub board: Option<String>,
    // TODO(kokiriglade): better color types here?
    /// The nation's hex dynmap colour.
    pub dynmap_colour: String,
    /// The nation's hex dynmap outline colour.
    pub dynmap_outline: String,
    /// The nation's wiki URL.
    pub wiki: Option<String>,
    /// The nation's leader (mayor of the capital town).
    #[serde(rename = "king")]
    pub leader: NamedId,
    /// The nation's capital town.
    pub capital: NamedId,
    pub timestamps: NationTimestamps,
    pub status: NationStatus,
    pub stats: NationStats,
    pub coordinates: NationCoordinates,
    /// A list of all residents of the nation.
    pub residents: Vec<NamedId>,
    /// A list of all towns of the nation.
    pub towns: Vec<NamedId>,
    /// A list of all allied nations of the nation.
    pub allies: Vec<NamedId>,
    /// A list of all enemy nations of the nation.
    pub enemies: Vec<NamedId>,
    /// A list of all sanctioned towns of the nation.
    pub sanctioned: Vec<NamedId>,
    pub ranks: BTreeMap<NationRankKind, Vec<String>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NationTimestamps {
    /// Unix timestamp representing when the nation was created.
    pub registered: i64,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NationStatus {
    /// If the nation is public.
    pub is_public: bool,
    /// If the nation is open (invite-less joining enabled).
    pub is_open: bool,
    /// If the nation is neutral.
    pub is_neutral: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NationStats {
    /// The total number of town blocks the nation has.
    pub num_town_blocks: i32,
    /// The current number of residents in the nation.
    pub num_residents: i32,
    /// The current number of towns in the nation.
    pub num_towns: i32,
    /// The current number of allied nations.
    pub num_allies: i32,
    /// The current number of enemy nations.
    pub num_enemies: i32,
    /// The nation's balance as seen on `/n` in-game.
    pub balance: f64,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NationCoordinates {
    /// The location of the nation's spawn.
    pub spawn: Location,
}

#[derive(
    Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord, Debug,
)]
#[serde(rename_all = "PascalCase")]
pub enum NationRankKind {
    Chancellor,
    Colonist,
    Diplomat,
}
