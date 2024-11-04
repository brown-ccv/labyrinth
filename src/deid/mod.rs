use crate::config::Action;
use anyhow::Result;
use base64::{engine, Engine};
use polars::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// The DeIdHashmaps struct is the single source of truth for the essential
// the following mappings:
//
//   ids: primary_id -> research_id,
//   salts: research_id -> salt,
//   dateshifts: research_id -> n_days
//
// In a given de-identification, there will only be one instance of the
// DeIdHashmaps struct.
#[derive(Debug)]
pub struct DeIdHashmaps {
    ids: HashMap<String, i32>,
    salts: HashMap<i32, String>,
    dateshifts: HashMap<i32, i32>,
    max_days: i32,
    shift_years: i32,
}

impl DeIdHashmaps {
    pub fn new(max_days: i32, shift_years: i32) -> Self {
        Self {
            ids: HashMap::new(),
            salts: HashMap::new(),
            dateshifts: HashMap::new(),
            max_days,
            shift_years,
        }
    }
}

pub fn deid_dataframe(
    df: &mut DataFrame,
    column_actions: &HashMap<String, Action>,
    columns_to_rename: &HashMap<String, String>,
) -> DataFrame {
    let mut remove_columns = vec![];
    let mut hash_columns = vec![];

    column_actions.iter().for_each(|(key, val)| match val {
        Action::Remove => remove_columns.push(key),
        Action::Hash => hash_columns.push(key),
        _ => (),
    });

    println!("Removing the following columns: {:?}", remove_columns);
    let mut df = df.drop_many(&remove_columns);

    hash_columns.iter().for_each(|col| {
        df.apply(col, hash_column_values);
    });

    // NOTE: The column re-naming MUST happen AFTER all other transformations and
    // removals. Otherwise, the renaming will prevent the transformations and removals
    // from happening as the orignal column names won't be found.
    columns_to_rename.iter().for_each(|(key, val)| {
        df.rename(key, val).unwrap();
    });

    df
}

fn hash_column_values(col: &Series) -> Series {
    println!("{:?}", col);

    col.iter().map(|elem| hash_value(elem).unwrap()).collect()
}

fn hash_value(input: AnyValue) -> Result<String> {
    let input_str = match input {
        AnyValue::UInt64(_) => format!("{:?}", input.try_extract::<u64>().unwrap()),
        AnyValue::UInt32(_) => format!("{:?}", input.try_extract::<u32>().unwrap()),
        AnyValue::UInt16(_) => format!("{:?}", input.try_extract::<u16>().unwrap()),
        AnyValue::UInt8(_) => format!("{:?}", input.try_extract::<u8>().unwrap()),
        AnyValue::Int64(_) => format!("{:?}", input.try_extract::<i64>().unwrap()),
        AnyValue::Int32(_) => format!("{:?}", input.try_extract::<i32>().unwrap()),
        AnyValue::Int16(_) => format!("{:?}", input.try_extract::<i16>().unwrap()),
        AnyValue::Int8(_) => format!("{:?}", input.try_extract::<i8>().unwrap()),
        AnyValue::Float64(_) => format!("{:?}", input.try_extract::<f64>().unwrap()),
        AnyValue::Float32(_) => format!("{:?}", input.try_extract::<f32>().unwrap()),
        AnyValue::Utf8(_) => input.get_str().unwrap().to_string(),
        _ => "".to_string(),
    };

    println!("{:?}", &input_str);

    let mut hasher = Sha256::new();
    hasher.update(input_str);

    let result = hasher.finalize();
    let encoded = engine::general_purpose::STANDARD.encode(result);

    Ok(encoded)
}
