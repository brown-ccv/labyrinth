use crate::config::Action;
use polars::prelude::*;
use std::collections::HashMap;

pub fn deid_dataframe(
    df: &mut DataFrame,
    column_actions: &HashMap<String, Action>,
    columns_to_rename: &HashMap<String, String>,
) -> DataFrame {
    let mut remove_columns = vec![];

    column_actions.into_iter().for_each(|(key, val)| match val {
        Action::Remove => remove_columns.push(key),
        _ => (),
    });

    columns_to_rename
        .into_iter()
        .for_each(|(key, val)| { df.rename(key, val).unwrap(); });

    println!("Removing the following columns: {:?}", remove_columns);

    df.drop_many(&remove_columns)
}
