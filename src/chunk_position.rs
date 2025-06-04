//! # Chunk Position
//!
//! Defines the [`ChunkPosition`] struct.
use std::fmt::{self, Formatter};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeTuple,
};

use crate::simple_position::SimplePosition;

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

impl From<ChunkPosition> for SimplePosition {
    fn from(chunk_pos: ChunkPosition) -> SimplePosition {
        SimplePosition {
            x: (chunk_pos.x * 16) as f32,
            y: (chunk_pos.y * 16) as f32,
        }
    }
}
