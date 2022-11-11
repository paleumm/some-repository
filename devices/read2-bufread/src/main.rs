use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("records.txt").unwrap();
    let mut buf_reader = BufReader::new(f);

    let mut buffer = String::new();
    buf_reader.read_line(&mut buffer).unwrap();
    println!("Read the following: {}", buffer);
}
