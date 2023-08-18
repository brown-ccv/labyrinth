use labyrinth::io;

#[test]
fn can_read_csv() {
    let df = io::read_csv("data/mock_data.csv").expect("Failed to read CSV file");

    assert_eq!(df.shape(), (1000, 9));
}
