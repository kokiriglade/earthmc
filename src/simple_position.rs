//! # Simple Position
//!
//! Defines the [`SimplePosition`] struct.
use std::fmt::{self, Formatter};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeTuple,
};

#[derive(Debug, PartialEq)]
pub struct SimplePosition {
    pub x: f32,
    pub y: f32,
}

impl Serialize for SimplePosition {
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

impl<'de> Deserialize<'de> for SimplePosition {
    fn deserialize<D>(deserializer: D) -> Result<SimplePosition, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TwoFloatsVisitor;

        impl<'de> Visitor<'de> for TwoFloatsVisitor {
            type Value = SimplePosition;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "an array of exactly two floats")
            }

            fn visit_seq<A>(
                self,
                mut seq: A,
            ) -> Result<SimplePosition, A::Error>
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

                Ok(SimplePosition { x, y })
            }
        }

        deserializer.deserialize_tuple(2, TwoFloatsVisitor)
    }
}
