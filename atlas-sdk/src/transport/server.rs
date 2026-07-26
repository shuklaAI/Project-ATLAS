use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct TransportServer;

impl TransportServer {
    pub fn start(port: u16) -> io::Result<()> {
        let listener = TcpListener::bind(("0.0.0.0", port))?;

        println!("========================================");
        println!("Transport server listening on port {}", port);
        println!("========================================");

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            Self::handle_client(stream);
                        });
                    }

                    Err(err) => {
                        eprintln!("Connection error: {}", err);
                    }
                }
            }
        });

        Ok(())
    }

    fn handle_client(mut stream: TcpStream) {
        println!(
            "\n[+] New TCP connection from {}",
            stream.peer_addr().unwrap()
        );

        let mut buffer = [0u8; 4096];

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!(
                        "[-] Client disconnected: {}",
                        stream.peer_addr().unwrap()
                    );
                    break;
                }

                Ok(size) => {
                    let message = String::from_utf8_lossy(&buffer[..size]);

                    println!(
                        "[RECV] {} -> {}",
                        stream.peer_addr().unwrap(),
                        message.trim()
                    );

                    if let Err(err) = stream.write_all(b"ACK\n") {
                        println!("Write error: {}", err);
                        break;
                    }

                    println!("[SEND] ACK");
                }

                Err(err) => {
                    println!("Read error: {}", err);
                    break;
                }
            }
        }
    }
}