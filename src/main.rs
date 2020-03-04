extern crate devserver_lib;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut address: String = "localhost:8000".to_string();

    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "--address" => {
                address = args
                    .next()
                    .expect("Pass an address with a port after the '--address' flag")
                    .to_string()
            }
            "--help" => {
                println!(
                    r#"Run 'devserver' in a folder to host that folder.

--address [address]:[port] Specify an address to use. The default is 'localhost:8000'.
--help                     Display the helpful information you're reading right now.

Examples:

devserver --address 127.0.0.1:8080

                "#
                );
                return;
            }
            _ => {}
        }
    }

    let path = env::current_dir().unwrap();

    println!(
        "Serving [{}] at [https://{}] or [http://{}] ",
        path.display(),
        address,
        address
    );
    devserver_lib::run(&address); // Runs forever serving the current folder on https://localhost:8000
}
