extern crate devserver_lib;
use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("Serving [{}] at https://localhost:8000", path.display());
    devserver_lib::run(); // Runs forever serving the current folder on https://localhost:8000
}
