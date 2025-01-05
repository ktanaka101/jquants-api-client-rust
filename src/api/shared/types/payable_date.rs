//! Types for representing payable dates.

use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

/// Represents a payable date.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "String")]
pub enum PayableDate {
    /// A date string.
    Date(String),
    /// "-": Undetermined
    Undetermined,
    /// "": Not applicable
    NotApplicable,
}

impl From<PayableDate> for String {
    fn from(payable_date: PayableDate) -> String {
        match payable_date {
            PayableDate::Date(value) => value,
            PayableDate::Undetermined => "-".to_string(),
            PayableDate::NotApplicable => "".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for PayableDate {
    fn deserialize<D>(deserializer: D) -> Result<PayableDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduledPaymentDateVisitor;

        impl Visitor<'_> for ScheduledPaymentDateVisitor {
            type Value = PayableDate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a date string, '-', or an empty string")
            }

            fn visit_str<E>(self, value: &str) -> Result<PayableDate, E>
            where
                E: de::Error,
            {
                match value {
                    "-" => Ok(PayableDate::Undetermined),
                    "" => Ok(PayableDate::NotApplicable),
                    _ => Ok(PayableDate::Date(value.to_string())),
                }
            }

            fn visit_string<E>(self, value: String) -> Result<PayableDate, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_str(ScheduledPaymentDateVisitor)
    }
}

impl PayableDate {
    /// Returns the variant of the payable date.
    pub fn variant(&self) -> String {
        match self {
            PayableDate::Date(_) => "Date".to_string(),
            PayableDate::Undetermined => "Undetermined".to_string(),
            PayableDate::NotApplicable => "NotApplicable".to_string(),
        }
    }

    /// Returns the date string if the payable date is a date.
    pub fn into_date(self) -> Option<String> {
        match self {
            PayableDate::Date(date) => Some(date),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::types::payable_date::PayableDate;

    #[test]
    fn test_serialize() {
        let amount_per_share = PayableDate::Date("2021-01-01".to_string());
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "2021-01-01");

        let amount_per_share = PayableDate::Date("2022-03-04".to_string());
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "2022-03-04");

        let amount_per_share = PayableDate::Undetermined;
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "-");

        let amount_per_share = PayableDate::NotApplicable;
        let serialized = serde_plain::to_string(&amount_per_share).unwrap();
        assert_eq!(serialized, "");
    }
}
