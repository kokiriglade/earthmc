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

/// An enum that can hold either a `String` or a [`Uuid`].
#[derive(Serialize, Clone)]
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

/// An API query that looks up by either UUIDs or names.
#[derive(Serialize, Builder)]
#[serde(transparent)]
#[builder(pattern = "owned")]
pub struct SimpleQuery {
    #[builder(default)]
    values: Vec<StrOrUuid>,
}

impl SimpleQueryBuilder {
    pub fn add<T: Into<StrOrUuid>>(mut self, single: T) -> Self {
        self.values.get_or_insert_with(Vec::new).push(single.into());
        self
    }
}

#[derive(Serialize, Builder)]
#[serde(transparent)]
#[builder(pattern = "owned")]
pub struct NearbyQuery {
    #[builder(default, setter(each = "add"))]
    values: Vec<NearbyQueryItem>,
}

#[derive(Debug, Serialize, Builder)]
#[builder(pattern = "owned")]
pub struct NearbyQueryItem {
    pub target_type: NearbyTargetType,
    pub target: NearbyTarget,
    pub search_type: NearbySearchType,
    pub radius: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NearbyTargetType {
    Town,
    Coordinate,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NearbySearchType {
    Town,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum NearbyTarget {
    Town(String),
    Coordinates([i32; 2]),
}
