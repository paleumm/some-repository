use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer).unwrap();
    io::stdout().write(&mut buffer.as_bytes()).unwrap();

    let mut buffer = [8; 1024];
    let stdin_handle = std::io::stdin();

    let mut locked_stdin_handle = stdin_handle.lock();

    locked_stdin_handle.read(&mut buffer).unwrap();

    let stdout_handle = std::io::stdout();

    let mut locked_stdout_handle = stdout_handle.lock();

    locked_stdout_handle.write(&mut buffer).unwrap();
}
