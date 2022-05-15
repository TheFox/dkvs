
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
use std::ops::Deref;
use std::ops::DerefMut;
use std::cell::Cell;
use std::cell::RefCell;

use crate::fs::config::Config;
use crate::net::client::Client;
use crate::utils::task::Task;
use crate::utils::task::Manager;

const CLIENT_INPUT_BUFFER_LEN: usize = 16;
const MAIN_WAIT: u64 = 1000;

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

        let mut manager = Manager::new(Duration::from_millis(MAIN_WAIT));
        manager.add_task("main".into(), Duration::new(10, 0), || {
            // println!("-> manager main()");
        });

        // Clients
        let mut clients_listener = TcpListener::bind(self.config.listen.clone())?;
        clients_listener
            .set_nonblocking(true)
            .expect("Cannot set Clients TcpListener to non-blocking");

        // Cluster
        // let cluster_listener: Option<TcpListener> = if self.config.cluster.enabled {
        //     let _cluster_listener = TcpListener::bind(self.config.listen.clone())?;
        //     _cluster_listener
        //         .set_nonblocking(true)
        //         .expect("Cannot set Clients TcpListener to non-blocking");
        //     Some(_cluster_listener)
        // }
        // else { None };

        while !self.shutdown {
            manager.start();

            // Tasks POC
            'clients_incoming_loop: for stream in clients_listener.incoming() {
                match stream {
                    Ok(mut client) => {
                        println!("-> stream OK");
                        dbg!(&clients_listener);
                        dbg!(&client);

                        match client.peer_addr() {
                            Ok(addr) => {
                                println!("-> new client: {:?}", addr);
                            },
                            Err(error) => panic!("Peer Addr: {}", error),
                        }

                        // let client2 = client.try_clone().expect("clone failed...");

                        self.tcp_clients.insert(self.tcp_clients_id, Client::from(client));
                        self.tcp_clients_id += 1;
                    },
                    Err(ref error) if error.kind() == ErrorKind::WouldBlock => {
                        // println!("-> WouldBlock: {}", error);
                        // sleep(Duration::from_millis(100));
                        // continue;
                        break 'clients_incoming_loop;
                    },
                    Err(error) => panic!("TcpListener encountered IO error: {}", error),
                }
            }

            // clients_listener = clients_listener.try_clone().expect("clone failed...");

            // Check Clients for input.
            let mut remove_tcp_clients: Vec<u64> = vec![];
            for (client_id, client) in &mut self.tcp_clients {
                let mut buffer = [0; CLIENT_INPUT_BUFFER_LEN];
                match client.tcp_stream.read(&mut buffer) {
                    Ok(_r) => {
                        println!("-> read OK");

                        let mut prev = [0; 2];
                        let mut buffer_pos = 0;
                        let mut args: Vec<String> = vec![];
                        while buffer_pos < CLIENT_INPUT_BUFFER_LEN {
                            println!("-> buffer[{}]: {:?} 0={} 1={}", buffer_pos, &buffer[buffer_pos], &prev[0], &prev[1]);

                            match buffer[buffer_pos] {
                                0 => {
                                    if buffer_pos == 2 && prev[0] == 0 && prev[1] == 0 {
                                        remove_tcp_clients.push(*client_id);
                                        break;
                                    }
                                    else if prev[0] == 13 && prev[1] == 10 {
                                        // New Line
                                        buffer_pos -= 2;
                                        break;
                                    }
                                },
                                _ => {},
                            }

                            prev[0] = prev[1];
                            prev[1] = buffer[buffer_pos];

                            buffer_pos += 1;
                        }

                        println!("-> prev: {:?}", prev);

                        let buffer_s: String = String::from_utf8_lossy(&buffer[0..buffer_pos]).to_string();
                        // let buffer_s: String = String::from_utf8(&buffer[0..buffer_pos]).unwrap();
                        println!("-> client input: l={} '{}'", &buffer_s.len(), &buffer_s);

                        match buffer_s.as_ref() {
                            "P" | "PING" => client.pong(),
                            "P" | "PING1" => {
                                let b = "2".to_string();
                                client.pong_id(&b);
                            },
                            "E" | "EXIT" | "Q" | "QUIT" => {
                                remove_tcp_clients.push(*client_id);
                            },
                            "S" | "SHUTDOWN" => self.shutdown = true,
                            _ => {},
                        }
                    },
                    Err(ref error) if error.kind() == ErrorKind::WouldBlock => {},
                    Err(error) => println!("-> read error: {:?}", error),
                    // Err(error) => {},
                }
            }

            for client_id in remove_tcp_clients {
                println!("-> remove client: {}", client_id);
                self.tcp_clients[&client_id].shutdown();
                self.tcp_clients.remove(&client_id);
            }

            println!("-> tcp_clients: {}", self.tcp_clients.len());

            // Task Management
            manager.run();
        }

        println!("-> Server::run done");
        Ok(())
    }
}
