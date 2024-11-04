use labyrinth::config::{Action, FileConfig};
use labyrinth::{deid, io};
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_drop_columns_from_dataframe() {
    let mut column_action_map = HashMap::new();
    column_action_map.insert("id".to_owned(), Action::Remove);
    column_action_map.insert("ip_address".to_owned(), Action::Remove);

    let mut columns_to_rename = HashMap::new();
    columns_to_rename.insert("first_name".to_string(), "firstname".to_string());

    let datetime_format = "".to_string();
    let file_config = FileConfig::new(
        Path::new("data/mock_data.csv"),
        &column_action_map,
        &columns_to_rename,
        datetime_format,
    );

    let mut raw_df = io::read_csv(file_config.filepath).expect("Failed to read CSV file");

    let df = deid::deid_dataframe(&mut raw_df, &column_action_map, &columns_to_rename);

    assert!(df.shape().1 == 7);
}

#[test]
fn test_rename_columns() {
    let mut column_action_map = HashMap::new();
    column_action_map.insert("id".to_owned(), Action::Remove);
    column_action_map.insert("ip_address".to_owned(), Action::Remove);

    let mut columns_to_rename = HashMap::new();
    columns_to_rename.insert("first_name".to_string(), "firstname".to_string());
    columns_to_rename.insert("last_name".to_string(), "lastname".to_string());

    let datetime_format = "".to_string();
    let file_config = FileConfig::new(
        Path::new("data/mock_data.csv"),
        &column_action_map,
        &columns_to_rename,
        datetime_format,
    );

    let mut raw_df = io::read_csv(file_config.filepath).expect("Failed to read CSV file");

    let df = deid::deid_dataframe(&mut raw_df, &column_action_map, &columns_to_rename);

    assert!(df.get_column_names().contains(&"firstname"));
}
