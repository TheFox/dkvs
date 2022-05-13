
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
use std::collections::HashMap;

use crate::fs::config::Config;
use crate::net::client::Client;
use crate::utils::task::Manager;

pub struct Server {
    config: Config,
    shutdown: bool,

    tcp_clients: HashMap<u64, Client>,
    tcp_clients_id: u64,
}

impl From<Config> for Server {
    fn from(config: Config) -> Self {
        Self {
            config: config,
            shutdown: false,

            tcp_clients: HashMap::new(),
            tcp_clients_id: 0,
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            shutdown: false,

            tcp_clients: HashMap::new(),
            tcp_clients_id: 0,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        println!("-> Server::run");
        println!("-> PID: {}", id());

        let mut manager = Manager::new();

        let ten_millis = Duration::from_millis(10);
        let fifty_millis = Duration::from_millis(50);
        let hundred_millis = Duration::from_millis(100);

        let clients_listener = TcpListener::bind(self.config.listen.clone())?;
        clients_listener.set_nonblocking(true).expect("Cannot set TcpListener non-blocking");

        let mut test1 = 0;
        manager.add_task("Test1".into(), Duration::new(5, 0));

        while !self.shutdown {
            println!("-> run test1: {}", test1);
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

                        // self.tcp_clients.push(Client::from(client));
                        self.tcp_clients.insert(self.tcp_clients_id, Client::from(client));
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

            let mut remove_tcp_clients: Vec<u64> = vec![];
            for (client_id, client) in &mut self.tcp_clients {
                let mut buffer = [0; 2048];
                match client.tcp_stream.read(&mut buffer) {
                    Ok(_r) => {
                        println!("-> read OK");

                        let mut prev = [0; 2];
                        let mut len = 0;
                        let mut args: Vec<String> = vec![];
                        while len < 2048 {
                            println!("-> buffer[{}]: {:?} 0={} 1={}", len, &buffer[len], &prev[0], &prev[1]);

                            match buffer[len] {
                                0 => {
                                    if prev[0] == 13 && prev[1] == 10 {
                                        len -= 2;
                                        break;
                                    }
                                },
                                _ => {},
                            }

                            prev[0] = prev[1];
                            prev[1] = buffer[len];

                            len += 1;
                        }

                        println!("-> prev: {:?}", prev);

                        let buffer_s: String = String::from_utf8_lossy(&buffer[0..len]).to_string();
                        // let buffer_s: String = String::from_utf8(&buffer[0..len]).unwrap();
                        println!("-> client input: l={} '{}'", &buffer_s.len(), &buffer_s);

                        match buffer_s.as_ref() {
                            "P" | "PING" => client.pong(),
                            "P" | "PING1" => {
                                let b = "2".to_string();
                                client.pong_id(&b);
                            },
                            "E" | "EXIT" => {
                                remove_tcp_clients.push(*client_id);
                                client.shutdown();
                            },
                            "S" | "SHUTDOWN" => self.shutdown = true,
                            _ => {},
                        }
                    },
                    // Err(error) => println!("-> read error: {}", error),
                    Err(error) => {},
                }
            }

            // println!("-> remove_tcp_clients: {}", remove_tcp_clients.len());
            for client_id in remove_tcp_clients {
                println!("-> remove client: {}", client_id);
                self.tcp_clients.remove(&client_id);
            }

            // Task Management
            let sleep_dur = if start_time.elapsed() < fifty_millis {
                fifty_millis - start_time.elapsed()
            }
            else {
                println!("-> time elapsed: {:?}", fifty_millis);
                Duration::from_millis(1)
            };
            sleep(sleep_dur);

            test1 += 1;
        }

        println!("-> Server::run done");
        Ok(())
    }
}
