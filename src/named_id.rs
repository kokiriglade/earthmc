//! # Named ID
//!
//! Defines the [`NamedIdGeneric`] structs for objects with just a name and an
//! ID.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An object with a name and an ID which can be specialized to `(String, Uuid)`
/// or `(Option<String>, Option<Uuid>)`.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NamedIdGeneric<N, U> {
    pub name: N,
    pub uuid: U,
}

pub type NamedId = NamedIdGeneric<String, Uuid>;

pub type NamedIdOpt = NamedIdGeneric<Option<String>, Option<Uuid>>;
