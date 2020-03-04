extern crate devserver_lib;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut https = false;
    let mut address: String = "localhost:8000".to_string();

    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "--https" => https = true,
            "--address" => {
                address = args
                    .next()
                    .expect("Pass an address with a port after the '--address' flag")
                    .to_string()
            }
            "--help" => {
                println!(
                    r#"Run 'devserver' in a folder to host that folder.

--https                    Enables https using a hardcoded self-signed certificate. This is obviously insecure for purposes other than local development!
--address [address]:[port] Specify an address to use. The default is 'localhost:8000'.
--help                     Display the helpful information you're reading right now.

Examples:

devserver --https --address 127.0.0.1:8080

                "#
                );
                return;
            }
            _ => {}
        }
    }

    let path = env::current_dir().unwrap();

    let prefix = if https { "https://" } else { "http://" };
    println!("Serving [{}] at [{}{}]", path.display(), prefix, address);
    devserver_lib::run(&address, https); // Runs forever serving the current folder on https://localhost:8000
}
