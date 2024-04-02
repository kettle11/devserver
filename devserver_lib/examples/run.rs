extern crate devserver_lib;

/// Hosts a server at http://localhost:8080 serving whatever folder this is run from.
fn main() {
    devserver_lib::run(&"localhost", 8080, "", false, "");
}
