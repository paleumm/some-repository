use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    let process = match Command::new("ps").stdout(Stdio::piped()).spawn() {
        Err(err) => panic!("could not spawn ps: {}", err),
        Ok(process) => process,
    };

    let mut ps_out = String::new();
    match process.stdout.unwrap().read_to_string(&mut ps_out) {
        Err(err) => panic!("could not read ps stdout: {}", err),
        Ok(_) => print!("ps output from child process is:\n{}", ps_out),
    }

    let process = match Command::new("rev")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(e) => panic!("could not spawn rev: {}", e),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all("palindrome".as_bytes()) {
        Err(e) => panic!("could not write to stdin: {}", e),
        Ok(_) => println!("sent text to rev command"),
    }

    let mut child_out = String::new();
    match process.stdout.unwrap().read_to_string(&mut child_out) {
        Err(e) => panic!("could not read stdout: {}", e),
        Ok(_) => println!("Output from child process is :\n{}", child_out),
    }
}
