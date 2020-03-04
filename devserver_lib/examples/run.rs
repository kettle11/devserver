extern crate devserver_lib;

/// Hosts a server at http://localhost:8000 serving whatever folder this is run from.
fn main() {
    devserver_lib::run("localhost:8000", false);
}
