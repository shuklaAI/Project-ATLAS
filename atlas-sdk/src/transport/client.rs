use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct TransportClient;

impl TransportClient {
    const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

    pub fn connect(address: &str, port: u16) -> io::Result<()> {
        let addr = (address, port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no address resolved"))?;

        println!("Connecting to {addr}...");

        let mut stream = TcpStream::connect_timeout(&addr, Self::CONNECT_TIMEOUT)?;
        stream.set_read_timeout(Some(Self::CONNECT_TIMEOUT))?;

        println!("Connected!");
        stream.write_all(b"Hello from Atlas!\n")?;
        println!("Sent greeting.");

        let mut buffer = [0u8; 1024];
        let size = stream.read(&mut buffer)?;
        println!("Server replied: {}", String::from_utf8_lossy(&buffer[..size]));

        Ok(())
    }
}