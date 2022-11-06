use std::process::Command;

fn main() {
    let out = Command::new("cat").arg("a.txt").output().unwrap();

    if !out.status.success() {
        println!("Command executed with failing error code");
    }

    println!("printing:\n{}", String::from_utf8(out.stdout).unwrap());
}
