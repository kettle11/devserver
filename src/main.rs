extern crate devserver_lib;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut address: String = "localhost:8080".to_string();
    let mut path: String = "".to_string();
    let mut headers = "".to_string();
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
            "--header" => {
                let mut new_header = args
                    .next()
                    .expect("Pass a header after the '--header' flag")
                    .to_string();
                if !new_header.contains(':') {
                    if new_header.contains('=') {
                        new_header = new_header.replacen("=", ":", 1);
                    } else {
                        panic!("Pass a ':' or '=' in the '--header' flag");
                    }
                }
                if new_header.contains('\r') || new_header.contains('\n') || !new_header.is_ascii()
                {
                    panic!("Only ASCII without line breaks is allowed in the '--header' flag");
                }
                headers.push_str("\r\n");
                headers.push_str(&new_header);
            }
            "--help" => {
                println!(
                    r#"Run 'devserver' in a folder to host that folder.

--reload                   Automatically refresh pages when a file in the hosted folder changes. Enabled by default.
--noreload                 Do not automatically refresh pages when a file in the hosted folder changes.
--address [address]:[port] Specify an address to use. The default is 'localhost:8080'.
--path [path]              Specify the path of the folder to be hosted.
--header                   Specify an additional header to send in responses. Use multiple --header flags for multiple headers.
--help                     Display the helpful information you're reading right now.

Examples:

devserver --address 127.0.0.1:8080 --path "some_directory/subdirectory" --header Access-Control-Allow-Origin='*'

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

    devserver_lib::run(
        &parts[0],
        port,
        &hosted_path.to_string_lossy(),
        reload,
        &headers,
    );
}
