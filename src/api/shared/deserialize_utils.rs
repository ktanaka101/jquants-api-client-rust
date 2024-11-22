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
