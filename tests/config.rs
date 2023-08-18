use labyrinth::config::{Action, FileConfig};
use std::path::Path;
use std::collections::HashMap;

#[test]
fn can_create_fileconfig() {
    let mut column_action_map = HashMap::new();
    column_action_map.insert("id".to_owned(), Action::Salt);
    let mut columns_to_rename = HashMap::new();
    columns_to_rename.insert("first_name".to_string(), "firstname".to_string());

    let datetime_format = "".to_string();
    let file_config = FileConfig::new(Path::new("config/mock_data.yml"), column_action_map, columns_to_rename, datetime_format);

    assert!(file_config.column_action_map.contains_key("id"));
}
