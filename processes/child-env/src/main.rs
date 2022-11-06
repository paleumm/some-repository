use std::process::Command;

fn main() {
    Command::new("env")
        .env("MY_PATH", "/tmp")
        .spawn()
        .expect("Command failed to execute");
    Command::new("env")
        .env_clear()
        .spawn()
        .expect("Command failed to execute");
}
