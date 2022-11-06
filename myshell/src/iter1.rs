use std::io::Write;
use std::io::{stdin, stdout};
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Unable to read user input");
        let command_to_exec = user_input.trim();
        let mut child = Command::new(command_to_exec)
            .spawn()
            .expect("Unable to execute command");
        child.wait().unwrap();
    }
}
