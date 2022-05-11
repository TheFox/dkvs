
use std::convert::From;
use std::net::TcpStream;
use std::net::Shutdown;
use std::io::Write;

#[derive(Debug)]
pub struct Client {
    pub tcp_stream: TcpStream,
}

impl From<TcpStream> for Client {
    fn from(tcp_stream: TcpStream) -> Self {
        Self {
            tcp_stream: tcp_stream,
        }
    }
}

impl Client {
    pub fn pong(&mut self) {
        let buffer = "PONG\r\n".as_bytes();
        self.tcp_stream.write_all(&buffer);
    }

    pub fn shutdown(&self) {
        self.tcp_stream.shutdown(Shutdown::Both).expect("TcpStream.shutdown() call failed");
    }
}
