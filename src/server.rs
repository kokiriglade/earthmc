//! # Server
//!
//! Defines the [`Server`] struct.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// The server's current version as a string
    pub version: String,
    /// The moon's current phase
    pub moon_phase: MoonPhase,
    /// Timestamp-related information for the server.
    pub timestamps: ServerTimestamps,
    /// Current weather status of the server.
    pub status: ServerStatus,
    /// Various statistics for the server.
    pub stats: ServerStats,
    /// Vote party-related information.
    pub vote_party: VoteParty,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimestamps {
    /// Time the new day occurs at.
    pub new_day_time: i64,
    /// The time of day, in seconds, in the server's current timezone.
    pub server_time_of_day: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatus {
    /// If the server is currently raining.
    pub has_storm: bool,
    /// If the server is currently thundering.
    pub is_thundering: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerStats {
    /// The amount of ticks that have passed within the current day.
    pub time: i64,
    /// The amount of ticks that have ever passed.
    pub full_time: i64,
    /// The total amount of players that can connect to the server.
    pub max_players: i32,
    /// The current amount of online players.
    pub num_online_players: i32,
    /// The current amount of online players with no town.
    pub num_online_nomads: i32,
    /// The total amount of currently registered Towny residents.
    pub num_residents: i32,
    /// The total amount of registered Towny residents who have no town.
    pub num_nomads: i32,
    /// The total amount of currently registered Towny towns.
    pub num_towns: i32,
    /// The total amount of town blocks across all towns.
    pub num_town_blocks: i32,
    /// The total amo9unt of currently registered Towny nations.
    pub num_nations: i32,
    /// The total amount of Quarters on the server.
    pub num_quarters: i32,
    /// The total amount of cuboids within all Quarters.
    pub num_cuboids: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VoteParty {
    /// The total votes required to trigger a vote party.
    pub target: i32,
    /// The votes remaining before a vote party is triggered.
    pub num_remaining: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoonPhase {
    FirstQuarter,
    FullMoon,
    LastQuarter,
    NewMoon,
    WaningCrescent,
    WaningGibbous,
    WaxingCrescent,
    WaxingGibbous,
}
