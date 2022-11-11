use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("records.txt").unwrap();

    let mut buffer = [0; 1024];

    let ff = f.read(&mut buffer[..]).unwrap();
    println!("{}", ff);
}
