use crate::config::Action;
use polars::prelude::*;
use std::collections::HashMap;

pub fn deid_dataframe(df: &DataFrame, column_actions: &HashMap<String, Action>) -> DataFrame {
    let mut remove_columns = vec![];

    column_actions.into_iter().for_each(|(key, val)| match val {
        Action::Remove => remove_columns.push(key),
        _ => (),
    });

    println!("Removing the following columns: {:?}", remove_columns);

    df.drop_many(&remove_columns) 
}
