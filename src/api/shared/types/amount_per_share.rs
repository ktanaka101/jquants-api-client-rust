//! Represents an amount per share.

use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

/// Represents an amount per share.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(into = "String")]
pub enum AmountPerShare {
    /// A number.
    Number(f64),
    /// "-": Undetermined
    Undetermined,
    /// "": Not applicable
    NotApplicable,
}

impl From<AmountPerShare> for String {
    fn from(amount_per_share: AmountPerShare) -> String {
        match amount_per_share {
            AmountPerShare::Number(value) => value.to_string(),
            AmountPerShare::Undetermined => "-".to_string(),
            AmountPerShare::NotApplicable => "".to_string(),
        }
    }
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

impl AmountPerShare {
    /// Returns the variant of the amount per share.
    pub fn variant(&self) -> String {
        match self {
            AmountPerShare::Number(_) => "Number".to_string(),
            AmountPerShare::Undetermined => "Undetermined".to_string(),
            AmountPerShare::NotApplicable => "NotApplicable".to_string(),
        }
    }

    /// Returns the number if the variant is `Number`.
    pub fn into_number(self) -> Option<f64> {
        match self {
            AmountPerShare::Number(value) => Some(value),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::types::amount_per_share::AmountPerShare;

    #[test]
    fn test_serialize() {
        let amount_per_share = AmountPerShare::Number(1.5);
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "1.5");

        let amount_per_share = AmountPerShare::Number(1.0);
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "1");

        let amount_per_share = AmountPerShare::Undetermined;
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "-");

        let amount_per_share = AmountPerShare::NotApplicable;
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "");
    }
}
