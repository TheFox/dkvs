
use std::convert::From;
use std::net::TcpStream;

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
