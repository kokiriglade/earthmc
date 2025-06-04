//! # Shared
//!
//! Defines common structs found throughout the API, such as [`NamedId`].
use std::fmt::{self, Formatter};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeTuple,
};
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WorldPosition {
    pub world: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Debug, PartialEq)]
pub struct SimpleWorldPosition {
    pub x: f32,
    pub y: f32,
}

impl Serialize for SimpleWorldPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SimpleWorldPosition {
    fn deserialize<D>(deserializer: D) -> Result<SimpleWorldPosition, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwoFloatsVisitor;

        impl<'de> Visitor<'de> for TwoFloatsVisitor {
            type Value = SimpleWorldPosition;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "an array of exactly two floats")
            }

            fn visit_seq<A>(
                self,
                mut seq: A,
            ) -> Result<SimpleWorldPosition, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let x: f32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let y: f32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                // if there’s a third element, that’s an error
                if let Some(_) = seq.next_element::<serde::de::IgnoredAny>()? {
                    return Err(de::Error::invalid_length(
                        3,
                        &"expected exactly two floats",
                    ));
                }

                Ok(SimpleWorldPosition { x, y })
            }
        }

        deserializer.deserialize_tuple(2, TwoFloatsVisitor)
    }
}

#[derive(Debug, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

impl Serialize for ChunkPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ChunkPosition {
    fn deserialize<D>(deserializer: D) -> Result<ChunkPosition, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwoFloatsVisitor;

        impl<'de> Visitor<'de> for TwoFloatsVisitor {
            type Value = ChunkPosition;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "an array of exactly two floats")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<ChunkPosition, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let x: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let y: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                if let Some(_) = seq.next_element::<serde::de::IgnoredAny>()? {
                    return Err(de::Error::invalid_length(
                        3,
                        &"expected exactly two ints",
                    ));
                }

                Ok(ChunkPosition { x, y })
            }
        }

        deserializer.deserialize_tuple(2, TwoFloatsVisitor)
    }
}

impl From<ChunkPosition> for SimpleWorldPosition {
    fn from(chunk_pos: ChunkPosition) -> SimpleWorldPosition {
        SimpleWorldPosition {
            x: (chunk_pos.x * 16) as f32,
            y: (chunk_pos.y * 16) as f32,
        }
    }
}
