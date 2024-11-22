use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::{fmt, str::FromStr};

/// Deserialize an empty string or null as None.
pub(crate) fn empty_string_or_null_as_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => T::from_str(&s).map(Some).map_err(D::Error::custom),
        None => Ok(None),
    }
}

/// Helper function to deserialize fields that can be either a number or a string.
/// If the field is a number, it returns the number as `Some(f64)`.
/// If the field is a string representing a number, it parses and returns `Some(f64)`.
/// If the field is "", or any non-numeric string, it returns `None`.
pub(crate) fn deserialize_f64_or_none<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct F64OrNoneVisitor;

    impl serde::de::Visitor<'_> for F64OrNoneVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float or a string representing a float")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value as f64))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value as f64))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value.trim() {
                "" => Ok(None),
                s => s.parse::<f64>().map(Some).map_err(serde::de::Error::custom),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&value)
        }
    }

    deserializer.deserialize_any(F64OrNoneVisitor)
}
