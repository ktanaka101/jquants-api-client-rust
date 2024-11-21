//! Types for representing payable dates.

use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

/// Represents a payable date.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PayableDate {
    /// A date string.
    Date(String),
    /// "-": Undetermined
    Undetermined,
    /// "": Not applicable
    NotApplicable,
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
