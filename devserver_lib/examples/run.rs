extern crate devserver_lib;

/// Hosts a server at https://localhost:8000 serving whatever folder this is run from.
/// The server has a hardcoded self-signed certificate, so no browser will trust it.
fn main() {
    devserver_lib::run("localhost:8000", false);
}
