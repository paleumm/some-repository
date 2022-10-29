use chrono::Utc;
use fLib::hello_from_lib;
fn main() {
    println!("Going to call library function {:?}", chrono::Utc::now());
    hello_from_lib("Rust system programmer");
}
