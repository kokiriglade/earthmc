//! # Player
//!
//! Defines the [`Player`] struct.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    named_id::{NamedId, NamedIdOpt},
    nation::NationRankKind,
    permission::TownyPermissions,
    town::TownRankKind,
};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// The player's name.
    pub name: String,
    /// The player's [`Uuid`].
    pub uuid: Uuid,
    /// The player's title set through `/n set title` in-game.
    pub title: Option<String>,
    /// The player's title set through `/n set surname` in-game.
    pub surname: Option<String>,
    /// Formatted name combining, in this order, title, username and surname.
    pub formatted_name: String,
    /// About section of `/res` set with `/res set about` in-game.
    pub about: Option<String>,
    /// The player's town.
    pub town: NamedIdOpt,
    /// The nation that the player's town is a part of.
    pub nation: NamedIdOpt,
    pub timestamps: PlayerTimestamps,
    pub status: PlayerStatus,
    pub stats: PlayerStats,
    pub perms: TownyPermissions,
    /// The ranks that the player has in both their town and nation.
    pub ranks: PlayerRanks,
    /// List of the player's friends.
    pub friends: Vec<NamedId>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTimestamps {
    /// Unix timestamp representing when the player joined the server.
    pub registered: i64,
    /// Unix timestamp representing when the player joined their town.
    pub joined_town_at: Option<i64>,
    /// Unix timestamp representing when the player was last online.
    /// Can be [`None`] if the player is an NPC.
    pub last_online: Option<i64>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatus {
    /// If the player is currently online.
    pub is_online: bool,
    /// If the player is a Towny NPC.
    #[serde(rename = "isNPC")]
    pub is_npc: bool,
    /// If the player is a mayor of a town.
    pub is_mayor: bool,
    /// If the player is a leader of a nation.
    pub is_king: bool,
    /// If the player is currently in a town.
    pub has_town: bool,
    /// If the player is currently in a nation.
    pub has_nation: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    /// The player's current balance as seen on `/res` in-game.
    pub balance: f64,
    /// The amount of friends this player has.
    pub num_friends: i32,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanks {
    /// A list of town ranks the player holds.
    pub town_ranks: Vec<TownRankKind>,
    /// A list of nation ranks the player holds.
    pub nation_ranks: Vec<NationRankKind>,
}
