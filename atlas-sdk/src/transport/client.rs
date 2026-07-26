use std::io::{self, Read, Write};
use std::net::TcpStream;

pub struct TransportClient;

impl TransportClient {
    pub fn connect(address: &str, port: u16) -> io::Result<()> {
        println!("Connecting to {}:{}...", address, port);

        let mut stream = TcpStream::connect((address, port))?;

        println!("Connected!");

        stream.write_all(b"Hello from Atlas!\n")?;

        println!("Sent greeting.");

        let mut buffer = [0u8; 1024];

        let size = stream.read(&mut buffer)?;

        println!(
            "Server replied: {}",
            String::from_utf8_lossy(&buffer[..size])
        );

        Ok(())
    }
}