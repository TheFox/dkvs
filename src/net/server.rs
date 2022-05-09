
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_client(stream: TcpStream) {
    println!("-> handle client");
}

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").ok();

        if let Some(_listener) = listener {
            // _listener.set_nonblocking(true);

            println!("-> listener ok");
            for stream in _listener.incoming() {
                handle_client(stream.ok().unwrap());
            }
        }
    }
}
