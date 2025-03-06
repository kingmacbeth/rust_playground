use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let json = File::open("whatever.json").expect("Unable to open file.");
    let mut reader = BufReader::new(json);

    let mut buffer = Vec::new();
    let size_in_bytes = reader.read_to_end(&mut buffer);

    println!("The size of the JSON file is: {:?} bytes", size_in_bytes);
}
