use polars::{
    prelude::{CategoricalOrdering, Column, PlSmallStr},
    series::{IntoSeries, Series},
};
use serde::Serialize;

pub fn build_column<T: Serialize>(
    name: PlSmallStr,
    values: Vec<T>,
) -> Result<Column, serde_plain::Error> {
    Ok(Column::from(build_series(name, values)?))
}

pub fn build_series<T: Serialize>(
    name: PlSmallStr,
    values: Vec<T>,
) -> Result<Series, serde_plain::Error> {
    let mut builder = polars::prelude::CategoricalChunkedBuilder::new(
        name,
        values.len(),
        CategoricalOrdering::Lexical,
    );

    for value in values {
        let s = serde_plain::to_string(&value).unwrap().to_string();
        builder.append_value(&s);
    }

    Ok(builder.finish().into_series())
}
