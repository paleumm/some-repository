use std::{fs::OpenOptions, path::PathBuf};
fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("./src/main.rs").unwrap();
    
        let hello = PathBuf::from("/tmp/hello.txt");
        hello.extension();
}
