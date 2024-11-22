//! Represents an amount per share.

use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

/// Represents an amount per share.
#[derive(Debug, Clone, PartialEq)]
pub enum AmountPerShare {
    /// A number.
    Number(f64),
    /// "-": Undetermined
    Undetermined,
    /// "": Not applicable
    NotApplicable,
}

impl<'de> Deserialize<'de> for AmountPerShare {
    fn deserialize<D>(deserializer: D) -> Result<AmountPerShare, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AmountPerShareVisitor;

        impl Visitor<'_> for AmountPerShareVisitor {
            type Value = AmountPerShare;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a float, '-', or an empty string")
            }

            fn visit_f64<E>(self, value: f64) -> Result<AmountPerShare, E>
            where
                E: de::Error,
            {
                Ok(AmountPerShare::Number(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<AmountPerShare, E>
            where
                E: de::Error,
            {
                match value {
                    "-" => Ok(AmountPerShare::Undetermined),
                    "" => Ok(AmountPerShare::NotApplicable),
                    _ => value
                        .parse::<f64>()
                        .map(AmountPerShare::Number)
                        .map_err(|_| E::custom(format!("Invalid number: {}", value))),
                }
            }
        }

        deserializer.deserialize_any(AmountPerShareVisitor)
    }
}
