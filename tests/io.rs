use labyrinth::io;
use std::path::Path;

#[test]
fn can_read_csv() {
    let df = io::read_csv(Path::new("data/mock_data.csv")).expect("Failed to read CSV file");

    assert_eq!(df.shape(), (1000, 9));
}
