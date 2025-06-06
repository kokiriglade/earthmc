//! # Permission
//!
//! Defines the [`TownyPermission`] struct.
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, IgnoredAny, SeqAccess, Visitor},
    ser::SerializeTuple,
};
use std::fmt::{self, Formatter};

/// Permission flags for each type of player in a given [`Town`].
#[derive(Debug, PartialEq)]
pub struct TownyPermissionSet {
    pub resident: bool,
    pub nation: bool,
    pub ally: bool,
    pub outsider: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TownyPermissions {
    pub build: TownyPermissionSet,
    pub destroy: TownyPermissionSet,
    pub switch: TownyPermissionSet,
    pub item_use: TownyPermissionSet,
    pub flags: TownyPermissionFlags,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TownyPermissionFlags {
    pub pvp: bool,
    pub explosion: bool,
    pub fire: bool,
    pub mobs: bool,
}

impl Serialize for TownyPermissionSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(4)?;
        seq.serialize_element(&self.resident)?;
        seq.serialize_element(&self.nation)?;
        seq.serialize_element(&self.ally)?;
        seq.serialize_element(&self.outsider)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for TownyPermissionSet {
    fn deserialize<D>(deserializer: D) -> Result<TownyPermissionSet, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermVisitor;

        impl<'de> Visitor<'de> for PermVisitor {
            type Value = TownyPermissionSet;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("an array of exactly four booleans")
            }

            fn visit_seq<A>(
                self,
                mut seq: A,
            ) -> Result<TownyPermissionSet, A::Error>
            where
                A: SeqAccess<'de>,
            {
                // pull out each boolean in order; if the array has fewer
                // than 4 items, error.
                let resident = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nation = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let ally = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let outsider = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                // if there are extra elements beyond the fourth, that’s also
                // an error
                if (seq.next_element::<IgnoredAny>()?).is_some() {
                    return Err(de::Error::invalid_length(
                        5,
                        &"expected exactly four booleans",
                    ));
                }

                Ok(TownyPermissionSet {
                    resident,
                    nation,
                    ally,
                    outsider,
                })
            }
        }

        deserializer.deserialize_tuple(4, PermVisitor)
    }
}
