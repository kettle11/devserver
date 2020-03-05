extern crate devserver_lib;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut address: String = "localhost:8080".to_string();
    let mut path: String = "".to_string();
    let mut args = args.iter();
    let mut reload = false;
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "--address" => {
                address = args
                    .next()
                    .expect("Pass an address with a port after the '--address' flag")
                    .to_string()
            }
            "--reload" => reload = true,
            "--path" => {
                path = args
                    .next()
                    .expect("Pass a path after the '--path' flag")
                    .to_string()
            }
            "--help" => {
                println!(
                    r#"Run 'devserver' in a folder to host that folder.

--reload                   Automatically refresh pages when a file in the hosted folder changes. Disabled by default.
--address [address]:[port] Specify an address to use. The default is 'localhost:8080'.
--path [path]              Specify the path of the folder to be hosted.
--help                     Display the helpful information you're reading right now.

Examples:

devserver --address 127.0.0.1:8080 --path "some_directory/subdirectory"

                "#
                );
                return;
            }
            _ => {}
        }
    }
    let hosted_path = env::current_dir().unwrap().join(Path::new(&path));

    if !std::path::Path::new(&hosted_path).exists() {
        println!("Path [{}] does not exist!", hosted_path.display());
        return;
    }

    println!(
        "Serving [{}] at [https://{}] or [http://{}] ",
        hosted_path.display(),
        address,
        address
    );

    if reload {
        println!("Automatic reloading is enabled!");
    }

    let parts: Vec<&str> = address.split(':').collect();
    let port = if let Some(port) = parts.get(1) {
        port.parse().expect("Port must be a number")
    } else {
        8080
    };

    devserver_lib::run(&parts[0], port, &hosted_path.to_string_lossy(), reload);
}
