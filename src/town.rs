//! # Town
//!
//! Defines the [`Town`] struct.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chunk_position::ChunkPosition,
    named_id::{NamedId, NamedIdOpt},
    permission::TownyPermissions,
    world_location::WorldLocation,
};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Town {
    /// The town's name.
    pub name: String,
    /// The town's [`Uuid`].
    pub uuid: Uuid,
    /// The town's board as seen on `/t` in-game.
    pub board: Option<String>,
    /// The founder of the town as seen on `/t` in-game.
    pub founder: String,
    /// The town's wiki URL.
    pub wiki: Option<String>,
    /// The town's mayor.
    pub mayor: NamedId,
    /// The town's nation.
    pub nation: NamedIdOpt,
    pub timestamps: TownTimestamps,
    pub status: TownStatus,
    pub stats: TownStats,
    pub perms: TownyPermissions,
    pub coordinates: TownCoordinates,
    /// A list of all of the residents of the town.
    pub residents: Vec<NamedId>,
    /// A list of all of the trusted residents of the town.
    pub trusted: Vec<NamedId>,
    /// A list of all of the outlawed people of the town.
    pub outlaws: Vec<NamedId>,
    /// A list of every Quarter in the town.
    pub quarters: Vec<NamedId>,
    /// Lists of everyone in every rank in the town.
    pub ranks: BTreeMap<TownRankKind, Vec<NamedId>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TownTimestamps {
    /// Unix timestamp representing when the town was created.
    pub registered: i64,
    /// Unix timestamp representing when the town joined its current nation.
    pub joined_nation_at: i64,
    /// Unix timestamp representing when the town fell into ruin.
    pub ruined_at: Option<i64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TownStatus {
    /// If the town is public.
    pub is_public: bool,
    /// If the town is open (invite-less joining enabled).
    pub is_open: bool,
    /// If the town is neutral.
    pub is_neutral: bool,
    /// If the town is the nation's capital.
    pub is_capital: bool,
    /// If the town ahs more claims than it should.
    pub is_over_claimed: bool,
    /// If the town is ruined.
    pub is_ruined: bool,
    /// If the town is for sale.
    pub is_for_sale: bool,
    /// If the town has a nation.
    pub has_nation: bool,
    /// If the town currently has an overclaim shield.
    pub has_overclaim_shield: bool,
    /// If the town allows outsiders to teleport to its spawn point.
    pub can_outsiders_spawn: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TownStats {
    /// The total number of town blocks the town has.
    pub num_town_blocks: i32,
    /// The maximum town blocks the town can claim.
    pub max_town_blocks: i32,
    /// The current number of residents in the town.
    pub num_residents: i32,
    /// The total number of trusted residents in the town.
    pub num_trusted: i32,
    /// The total number of players that are outlawed in the town.
    pub num_outlaws: i32,
    /// The town's balance as seen on `/t` in-game.
    pub balance: f64,
    /// The price the town is for sale at if it is for sale.
    pub for_sale_price: Option<f64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TownCoordinates {
    /// The locatioin of the town's spawn point.
    pub spawn: WorldLocation,
    pub home_block: ChunkPosition,
    pub town_blocks: Vec<ChunkPosition>,
}

#[derive(
    Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord, Debug,
)]
#[serde(rename_all = "PascalCase")]
pub enum TownRankKind {
    Councillor,
    Builder,
    Recruiter,
    Police,
    #[serde(rename = "Tax-exempt")]
    TaxExempt,
    Treasurer,
    Realtor,
    Settler,
}
