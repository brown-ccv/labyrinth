use polars_core::prelude::*;
use polars_io::prelude::*;

pub fn read_csv(filename: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(filename)?.has_header(true).finish()
}
