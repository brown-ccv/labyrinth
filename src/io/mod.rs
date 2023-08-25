use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn read_csv(filepath: &Path) -> PolarsResult<DataFrame> {
    CsvReader::from_path(filepath)?.has_header(true).finish()
}

pub fn write_csv(filepath: &Path, df: &mut DataFrame) {
    let mut file = File::create(filepath).unwrap();
    CsvWriter::new(&mut file).finish(df).unwrap();

    println!("{} has been successfully written!", filepath.display());
}
