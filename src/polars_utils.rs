use std::collections::{BTreeSet, HashMap};

use polars::{
    prelude::{CategoricalOrdering, Column, PolarsError},
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

pub fn build_categorical_column<T: Serialize>(
    name: &str,
    values: Vec<T>,
) -> Result<Column, IntoPolarsError> {
    Ok(Column::from(build_categorical_series(name, values)?))
}

pub fn build_categorical_series<T: Serialize>(
    name: &str,
    values: Vec<T>,
) -> Result<Series, IntoPolarsError> {
    let mut builder = polars::prelude::CategoricalChunkedBuilder::new(
        name.into(),
        values.len(),
        CategoricalOrdering::Lexical,
    );

    for value in values {
        let s = serde_plain::to_string(&value)?.to_string();
        builder.append_value(&s);
    }

    Ok(builder.finish().into_series())
}

pub fn hashmap_list_to_columns(map_list: Vec<HashMap<String, String>>) -> Vec<Column> {
    let mut all_keys = BTreeSet::new();
    for map in &map_list {
        for key in map.keys() {
            all_keys.insert(key.clone());
        }
    }

    let mut columns = Vec::with_capacity(all_keys.len());

    enum ColumnType {
        Boolean,
        Float,
        String,
    }

    for key in &all_keys {
        let first_non_empty_value = map_list
            .iter()
            .flat_map(|m| m.get(key))
            .find(|s| !s.is_empty());

        let can_parse_type = match first_non_empty_value {
            Some(val) => {
                if val.parse::<f64>().is_ok() {
                    ColumnType::Float
                } else if val.parse::<bool>().is_ok() {
                    ColumnType::Boolean
                } else {
                    ColumnType::String
                }
            }
            None => ColumnType::String,
        };

        match can_parse_type {
            ColumnType::Boolean => {
                let col_data: Vec<Option<bool>> = map_list
                    .iter()
                    .map(|m| match m.get(key) {
                        Some(s) => s.parse::<bool>().ok(),
                        None => None,
                    })
                    .collect();
                let col = Column::new(key.into(), col_data);
                columns.push(col);
                continue;
            }
            ColumnType::Float => {
                let col_data: Vec<Option<f64>> = map_list
                    .iter()
                    .map(|m| match m.get(key) {
                        Some(s) => s.parse::<f64>().ok(),
                        None => None,
                    })
                    .collect();
                let col = Column::new(key.into(), col_data);
                columns.push(col);
                continue;
            }
            ColumnType::String => {
                let col_data: Vec<Option<String>> =
                    map_list.iter().map(|m| m.get(key).cloned()).collect();
                let col = Column::new(key.into(), col_data);
                columns.push(col);
                continue;
            }
        }
    }

    columns
}
