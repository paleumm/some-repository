use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let f = File::create("file.txt").unwrap();

    let mut buf_writer = BufWriter::new(f);

    let buffer = String::from("hello, testing");

    buf_writer.write(buffer.as_bytes()).unwrap();
    println!("Wrote the folloing: {}", buffer);
}
