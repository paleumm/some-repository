use std::fs::File;
use std::io::Read;

fn read_files(handle: &mut impl Read) -> std::io::Result<String> {
    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let mut chained_handle;

    let file1 = "file1.txt";
    let file2 = "file3.txt";
    if let Ok(f1) = File::open(file1) {
        if let Ok(f2) = File::open(file2) {
            chained_handle = f1.chain(f2);
            let content = read_files(&mut chained_handle);
            match content {
                Ok(text) => println!("read from chained handle:n{}", text),
                Err(e) => println!("Error occured in reading files: {}", e),
            }
        } else {
            println!("Unable to read {}", file2);
        }
    } else {
        println!("Unable to read {}", file1);
    }
}
