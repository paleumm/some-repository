use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}:{}", key, value);
    }

    println!("Value of size is {}", env::var("size").unwrap());

    let args: Vec<String> = env::args().collect();
    let size = &args[1];
    let mode = &args[2];
    let source_folder = &args[3];
    println!(
        "Size:{},mode:{},source folder: {}",
        size, mode, source_folder
    );
}
