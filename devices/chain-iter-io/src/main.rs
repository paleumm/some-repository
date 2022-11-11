use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("file.txt").unwrap();
    let file_reader = BufReader::new(f);

    for single_line in file_reader.lines() {
        println!("Line read from file: {}", single_line.unwrap());
    }

    let s = std::io::stdin();

    let file_reader = BufReader::new(s);

    for single_line in file_reader.lines() {
        println!("You typed:{}", single_line.unwrap());
    }
}
