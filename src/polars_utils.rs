use polars::{
    prelude::{CategoricalOrdering, Column, PlSmallStr, PolarsError},
    series::{IntoSeries, Series},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntoPolarsError {
    #[error("Error converting to Polars: {0}")]
    SerializeError(#[from] serde_plain::Error),
    #[error("Error converting to Polars: {0}")]
    PolarsError(#[from] PolarsError),
}

pub fn build_column<T: Serialize>(
    name: PlSmallStr,
    values: Vec<T>,
) -> Result<Column, IntoPolarsError> {
    Ok(Column::from(build_series(name, values)?))
}

pub fn build_series<T: Serialize>(
    name: PlSmallStr,
    values: Vec<T>,
) -> Result<Series, IntoPolarsError> {
    let mut builder = polars::prelude::CategoricalChunkedBuilder::new(
        name,
        values.len(),
        CategoricalOrdering::Lexical,
    );

    for value in values {
        let s = serde_plain::to_string(&value)?.to_string();
        builder.append_value(&s);
    }

    Ok(builder.finish().into_series())
}
