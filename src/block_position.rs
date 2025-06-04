//! # Block Position
//!
//! Defines the [`BlockPosition`] struct.
use std::fmt::{self, Formatter};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeTuple,
};

#[derive(Debug, PartialEq)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Serialize for BlockPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(3)?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.serialize_element(&self.z)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BlockPosition {
    fn deserialize<D>(deserializer: D) -> Result<BlockPosition, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwoFloatsVisitor;

        impl<'de> Visitor<'de> for TwoFloatsVisitor {
            type Value = BlockPosition;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "an array of exactly three floats")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<BlockPosition, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let x: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let y: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let z: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                if let Some(_) = seq.next_element::<serde::de::IgnoredAny>()? {
                    return Err(de::Error::invalid_length(
                        4,
                        &"expected exactly three ints",
                    ));
                }

                Ok(BlockPosition { x, y, z })
            }
        }

        deserializer.deserialize_tuple(3, TwoFloatsVisitor)
    }
}
