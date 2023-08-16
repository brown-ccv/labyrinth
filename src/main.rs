use polars_core::prelude::*;
use polars_io::prelude::*;

fn read_csv(filename: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(filename)?.has_header(true).finish()
}

fn main() {
    let df = read_csv("data/mock_data.csv").expect("Failed to read CSV file");

    println!("{:?}", df);
}
