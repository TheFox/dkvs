
include!(concat!(env!("OUT_DIR"), "/config.rs"));

use std::env::args;
use std::io::Result;

use dkvs::app::App;
use dkvs::net::server::Server;
use dkvs::fs::config::Config;

#[allow(dead_code)]
fn print_app_info() {
    println!("{} v{} ({})", APP_NAME, APP_VERSION, APP_BUILD_AT);
    println!("{}", APP_AUTHORS);
    println!("{}", APP_HOMEPAGE);
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!("  dkvs_server [<OPTIONS...>]");
    println!();
    println!("Options:");
    println!("  -h|--help                       Show help.");
    println!("  -V|--version                    Show version.");
    println!("  -c|--config <path>              Path to server.json file.");
    println!();
}

/// Main
fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    println!("-> start");

    let args: Vec<String> = args().collect();
    let argc = args.len();

    if cfg!(debug_assertions) {
        println!("-> args: {:?}", args);
        println!("-> argc: {:?}", argc);
    }

    if argc == 1 {
        print_app_info();
        print_usage();
        return Ok(());
    }

    let mut app = App::new();
    let mut skip_next = false;
    for index in 1..argc {
        if skip_next {
            skip_next = false;
            continue;
        }
        let arg = &args[index];
        let next = &args.get(index + 1);

        #[cfg(debug_assertions)]
        println!("-> arg: #{} {:?}", index, arg);

        match arg.as_str() {
            "-h" | "--help" => {
                print_app_info();
                print_usage();
                return Ok(());
            },
            "-V" | "--version" => {
                print_app_info();
                print_usage();
                return Ok(());
            },
            "-c" => {
                if let Some(_next) = next {
                    app.config_file_path = Some(_next.to_string());
                    skip_next = true;
                }
            },
            _ => {
                panic!("Unrecognized argument: {}", arg);
            },
        }
    }

    if cfg!(debug_assertions) {
        println!("-> app.config_file_path: {:?}", app.config_file_path);
    }

    let server = Server::new();
    server.run();

    println!("-> end");

    Ok(())
}
