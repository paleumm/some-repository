use std::fs::OpenOptions;
fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("./src/main.rs")?;
}
