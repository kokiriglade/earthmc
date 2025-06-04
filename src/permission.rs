use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, IgnoredAny, SeqAccess, Visitor},
    ser::SerializeTuple,
};
use std::fmt::{self, Formatter};

/// Permission flags for each type of player in a given [`Town`].
#[derive(Debug, PartialEq)]
pub struct TownyPermission {
    pub resident: bool,
    pub nation: bool,
    pub ally: bool,
    pub outsider: bool,
}

impl Serialize for TownyPermission {
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

impl<'de> Deserialize<'de> for TownyPermission {
    fn deserialize<D>(deserializer: D) -> Result<TownyPermission, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermVisitor;

        impl<'de> Visitor<'de> for PermVisitor {
            type Value = TownyPermission;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("an array of exactly four booleans")
            }

            fn visit_seq<A>(
                self,
                mut seq: A,
            ) -> Result<TownyPermission, A::Error>
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
                if let Some(_) = seq.next_element::<IgnoredAny>()? {
                    return Err(de::Error::invalid_length(
                        5,
                        &"expected exactly four booleans",
                    ));
                }

                Ok(TownyPermission {
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
