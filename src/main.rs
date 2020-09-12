extern crate devserver_lib;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut address: String = "localhost:8080".to_string();
    let mut path: String = "".to_string();
    let mut args = args.iter();
    let mut reload = true;
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "--address" => {
                address = args
                    .next()
                    .expect("Pass an address with a port after the '--address' flag")
                    .to_string()
            }
            "--reload" | "--refresh" => reload = true,
            "--noreload" | "--norefresh" => reload = false,
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
            _ => {
                println!(
                    "Unrecognized flag: `{:?}`.\nSee available options with `devserver --help`",
                    arg
                );
                return;
            }
        }
    }
    let hosted_path = env::current_dir().unwrap().join(Path::new(&path));

    if !std::path::Path::new(&hosted_path).exists() {
        println!("Path [{}] does not exist!", hosted_path.display());
        return;
    }

    let parts: Vec<&str> = address.split(':').collect();
    let port = if let Some(port) = parts.get(1) {
        let port = port.parse();
        if let Ok(port) = port {
            port
        } else {
            println!("Error: Port must be a number");
            return;
        }
    } else {
        8080
    };

    println!(
        "\nServing [{}] at [ https://{} ] or [ http://{} ]",
        hosted_path.display(),
        address,
        address
    );

    if reload {
        println!("Automatic reloading is enabled!");
    }

    println!("Stop with Ctrl+C");

    devserver_lib::run(&parts[0], port, &hosted_path.to_string_lossy(), reload);
}
