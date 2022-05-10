
use std::io::Read;
use std::io::ErrorKind;
use std::io::Error;
use std::convert::From;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::id;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use crate::fs::config::Config;
use crate::net::client::Client;

pub struct Server {
    config: Config,
    shutdown: bool,

    tcp_clients: Vec<Client>,
}

impl From<Config> for Server {
    fn from(config: Config) -> Self {
        Self {
            config: config,
            shutdown: false,

            tcp_clients: vec![],
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            shutdown: false,

            tcp_clients: vec![],
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        println!("-> Server::run");
        println!("-> PID: {}", id());

        let ten_millis = Duration::from_millis(10);
        let fifty_millis = Duration::from_millis(50);
        let hundred_millis = Duration::from_millis(100);

        let clients_listener = TcpListener::bind(self.config.listen.clone())?;
        clients_listener.set_nonblocking(true).expect("Cannot set TcpListener non-blocking");

        while !self.shutdown {
            let start_time = Instant::now();

            'incoming_loop: for stream in clients_listener.incoming() {
                // dbg!(&stream);
                match stream {
                    Ok(mut client) => {
                        println!("-> stream OK");
                        dbg!(&client);

                        match client.peer_addr() {
                            Ok(addr) => {
                                println!("-> new client: {:?}", addr);
                            },
                            Err(error) => panic!("Peer Addr: {}", error),
                        }

                        self.tcp_clients.push(Client::from(client));



                        // println!("-> sleep");
                        // sleep(Duration::from_millis(2000));

                        // self.shutdown = true;
                    },
                    Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                        // println!("-> WouldBlock: {}", error);
                        // sleep(hundred_millis);
                        // continue;
                        break 'incoming_loop;
                    },
                    Err(error) => panic!("TcpListener encountered IO error: {}", error),
                }
            }

            println!("-> clients: {}", self.tcp_clients.len());
            for client in &mut self.tcp_clients {
                println!("-> client: {:?}", client);

                let mut buffer = [0; 1024];
                match client.tcp_stream.read(&mut buffer) {
                    Ok(_r) => {
                        println!("-> read OK: {:?}", _r);
                        let buffer_s = String::from_utf8_lossy(&buffer[..]);
                        println!("-> client says: '{}'", buffer_s);
                    },
                    // Err(error) => println!("-> read error: {}", error),
                    Err(error) => {},
                }
            }

            // println!("-> millis: {}", start_time.elapsed().as_millis());
            // println!("-> micros: {}", start_time.elapsed().as_micros());
            // println!("-> nanos: {}", start_time.elapsed().as_nanos());
            // println!("-> has 50ms? -> {}", start_time.elapsed() > fifty_millis);

            // let do_steps = || -> Result<(), Error> {
            //     let sleep_dur = start_time.elapsed() - fifty_millis;
            //     Ok(())
            // };
            // let res = do_steps();
            // dbg!(&res);

            // let sleep_dur = fifty_millis - start_time.elapsed();
            // println!("-> diff: {:?}", sleep_dur);

            let sleep_dur = if start_time.elapsed() < fifty_millis {
                fifty_millis - start_time.elapsed()
            }
            else {
                println!("-> time elapsed: {:?}", fifty_millis);
                Duration::from_millis(1)
            };

            // println!("-> sleep: {:?}", sleep_dur);
            sleep(sleep_dur);
        }

        println!("-> Server::run done");
        Ok(())
    }
}
