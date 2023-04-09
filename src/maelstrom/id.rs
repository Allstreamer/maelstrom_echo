// This file is part of Maelstrom Echo which is released under GNU GPL v2.0.
// See file LICENSE.

use serde::{
    de::Visitor, Deserialize, Deserializer, Serialize, Serializer,
};
use std::{fmt::Display, num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum IDParseError {
    #[error("Failed to parse ID number to u32")]
    FailedParseInt(#[from] ParseIntError),

    #[error("Failed to parse ID because it was Empty")]
    EmptyID,

    #[error("Failed to parse ID because it was Empty")]
    UnknownIDType,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ID {
    Node(u32),
    Client(u32),
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<ID, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IDVisitor)
    }
}

struct IDVisitor;
impl<'de> Visitor<'de> for IDVisitor {
    type Value = ID;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("an str")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match ID::from_str(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(E::custom(e)),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match ID::from_str(v.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(E::custom(e)),
        }
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ID::Node(v) => format!("n{}", v),
                ID::Client(v) => format!("c{}", v),
            }
        )
    }
}

impl FromStr for ID {
    type Err = IDParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();
        let client_number =
            input.chars().filter(|v| v.is_numeric()).collect::<String>();
        let client_number = client_number.parse::<u32>()?;

        let id_type = match input.chars().next() {
            Some(v) => v.to_lowercase().to_string(),
            None => return Err(IDParseError::EmptyID),
        };

        Ok(match id_type.as_str() {
            "n" => ID::Node(client_number),
            "c" => ID::Client(client_number),
            _ => return Err(IDParseError::UnknownIDType),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::maelstrom::id::*;

    macro_rules! test_str_to_id {
        ($typed: expr, $str: expr) => {
            assert_eq!(ID::from_str($str), $typed);
        };
    }

    #[test]
    fn str_to_id() {
        test_str_to_id!(Ok(ID::Client(0)), "c0");
        test_str_to_id!(Ok(ID::Client(1)), "c1");
        test_str_to_id!(Ok(ID::Client(2)), "c2");
        test_str_to_id!(Ok(ID::Client(10)), "c10");
        test_str_to_id!(Ok(ID::Client(1546543)), "c1546543");
        test_str_to_id!(Ok(ID::Client(u32::MAX)), "c4294967295");

        test_str_to_id!(Ok(ID::Node(0)), "n0");
        test_str_to_id!(Ok(ID::Node(1)), "n1");
        test_str_to_id!(Ok(ID::Node(2)), "n2");
        test_str_to_id!(Ok(ID::Node(10)), "n10");
        test_str_to_id!(Ok(ID::Node(1546543)), "n1546543");
        test_str_to_id!(Ok(ID::Node(u32::MAX)), "n4294967295");
    }

    #[test]
    fn id_to_str() {
        assert_eq!(ID::Client(0).to_string(), "c0");
        assert_eq!(ID::Client(1).to_string(), "c1");
        assert_eq!(ID::Client(2).to_string(), "c2");
        assert_eq!(ID::Client(10).to_string(), "c10");
        assert_eq!(ID::Client(1546543).to_string(), "c1546543");
        assert_eq!(ID::Client(u32::MAX).to_string(), "c4294967295");

        assert_eq!(ID::Node(0).to_string(), "n0");
        assert_eq!(ID::Node(1).to_string(), "n1");
        assert_eq!(ID::Node(2).to_string(), "n2");
        assert_eq!(ID::Node(10).to_string(), "n10");
        assert_eq!(ID::Node(1546543).to_string(), "n1546543");
        assert_eq!(ID::Node(u32::MAX).to_string(), "n4294967295");
    }
}
