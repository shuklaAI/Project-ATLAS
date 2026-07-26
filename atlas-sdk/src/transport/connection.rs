use std::net::TcpStream;

pub struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}