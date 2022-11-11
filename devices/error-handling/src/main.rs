use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let f1 = File::open("file1.txt")?;
    let f2 = File::open("file3.txt")?;

    let mut chained_handle = f1.chain(f2);

    let mut buffer = String::new();

    chained_handle.read_to_string(&mut buffer)?;
    println!("Read from chained handle: {}", buffer);

    Ok(())
}
