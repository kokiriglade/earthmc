//! # World
//!
//! Defines the [`World`] enum.
use std::fmt;

#[derive(Debug, Clone)]
pub enum World {
    Aurora,
    Other(String),
}

impl World {
    pub fn as_string(&self) -> String {
        match self {
            World::Aurora => "aurora".to_owned(),
            World::Other(name) => name.clone(),
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_string())
    }
}
