//! # Query
//!
//! Defines structs and enums used for querying specific information from the API.
use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub(crate) struct Query<D>
where
    D: Serialize + ?Sized,
{
    query: D,
}

impl<D: Serialize + Sized> From<D> for Query<D> {
    fn from(value: D) -> Self {
        Self { query: value }
    }
}

/// An enum that can hold either a `String` or a `Uuid`.
#[derive(Serialize)]
#[serde(untagged)]
pub enum StrOrUuid {
    Str(String),
    Uid(Uuid),
}

impl From<&str> for StrOrUuid {
    fn from(s: &str) -> Self {
        StrOrUuid::Str(s.to_string())
    }
}

impl From<String> for StrOrUuid {
    fn from(s: String) -> Self {
        StrOrUuid::Str(s)
    }
}

impl From<Uuid> for StrOrUuid {
    fn from(u: Uuid) -> Self {
        StrOrUuid::Uid(u)
    }
}

#[derive(Serialize, Builder)]
#[serde(transparent)]
#[builder(pattern = "owned", setter(into))]
pub struct TownQuery {
    #[builder(default, setter(each = "town"))]
    towns: Vec<StrOrUuid>,
}
