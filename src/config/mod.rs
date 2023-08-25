use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub enum Action {
    Salt, // Salt also implies hashing
    Hash,
    DateShift,
    Remove,
}

#[derive(Debug)]
pub struct FileConfig<'a> {
    pub filename: &'a str,
    pub filepath: &'a Path,
    pub column_action_map: &'a HashMap<String, Action>,
    pub columns_to_rename: HashMap<String, String>,
    pub datetime_format: String,
}

impl<'a> FileConfig<'a> {
    pub fn new(
        filepath: &'a Path,
        column_action_map: &'a HashMap<String, Action>,
        columns_to_rename: HashMap<String, String>,
        datetime_format: String,
    ) -> Self {
        let filename = filepath.file_name().unwrap().to_str().unwrap();

        Self {
            filename,
            filepath,
            column_action_map,
            columns_to_rename,
            datetime_format,
        }
    }
}
