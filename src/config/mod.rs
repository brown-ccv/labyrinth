use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub enum Action {
    Salt,
    Hash,
    DateShift,
    Remove,
}

#[derive(Debug)]
pub struct FileConfig<'a> {
    pub filename: &'a Path,
    pub column_action_map: HashMap<String, Action>,
    pub columns_to_rename: HashMap<String, String>,
    pub datetime_format: String,
}

impl <'a> FileConfig<'a> {
    pub fn new(
        filename: &'a Path,
        column_action_map: HashMap<String, Action>,
        columns_to_rename: HashMap<String, String>,
        datetime_format: String,
    ) -> Self {
        Self {
            filename,
            column_action_map,
            columns_to_rename,
            datetime_format,
        }
    }
}
