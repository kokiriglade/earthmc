//! `earthmc` is an async Rust client to interact with the
//! [EarthMC](https://earthmc.net) API.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! earthmc = "*"
//! ```
//!
//! Replace the `*` with the actual version you want to use.
//!
//!
//! Alternatively you can run:
//!
//! ```bash
//! cargo add earthmc
//! ````
//!
//! ## Usage
//!
//! ### Create a new client
//!
//! ```rust
//! # use earthmc::Client;
//! #
//! let client = Client::default();
//! ```
//!
//! Finally, for more advanced configurations you can use a `ClientBuilder`:
//!
//! ```rust
//! # use earthmc::{ClientBuilder, world::World};
//! #
//! let client = ClientBuilder::default()
//!     .world(World::Other("nostra".to_string()))
//!     .build()
//!     .unwrap();
//! ```
//!
//! Detailed usage examples are in the `examples` directory.
pub mod client;
pub mod discord_link;
pub mod errors;
pub mod location;
pub mod mystery_master;
pub mod named_id;
pub mod nation;
pub mod permission;
pub mod player;
pub mod player_stats;
pub mod quarter;
pub mod query;
pub mod retry_strategy;
pub mod server;
pub mod town;
pub mod world;
pub mod world_location;

pub use client::{Client, ClientBuilder};
pub use reqwest;
