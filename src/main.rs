use labyrinth::io;


fn main() {
    let df = io::read_csv("data/mock_data.csv").expect("Failed to read CSV file");

    println!("{:?}", df);
}
