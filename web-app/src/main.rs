use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    todolist_entries: Mutex<Vec<TodolsitEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolsitEntry {
    id: i32,
    date: i64,
    title: String,
}

fn main() {
    println!("Hello, World")
}
