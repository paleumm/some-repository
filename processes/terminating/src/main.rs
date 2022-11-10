use std::process;

fn main() {
    println!("Going to abort process");
    process::abort();
    // process::exit(64);

    println!("Process aborted");
}
