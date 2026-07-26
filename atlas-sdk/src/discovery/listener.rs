use super::{DiscoveryPacket, Peer, PeerRegistry};

use crate::transport::TransportClient;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Listener;

impl Listener {
    pub fn start(
        local_device_id: String,
        registry: Arc<Mutex<PeerRegistry>>,
    ) {
        thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:47000")
                .expect("Failed to bind UDP listener");

            println!("Discovery listener started on UDP 47000");

            let mut buffer = [0u8; 4096];

            loop {
                match socket.recv_from(&mut buffer) {
                    Ok((size, addr)) => {
                        let text = String::from_utf8_lossy(&buffer[..size]);

                        match serde_json::from_str::<DiscoveryPacket>(&text) {
                            Ok(packet) => {
                                // Ignore our own broadcasts
                                if packet.id == local_device_id {
                                    continue;
                                }

                                let peer = Peer {
                                    id: packet.id,
                                    name: packet.name,
                                    address: addr,
                                    last_seen: SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs(),
                                };

                                let mut registry = registry.lock().unwrap();

                                // Check whether we've already seen this peer
                                let first_time = !registry
                                    .peers()
                                    .iter()
                                    .any(|p| p.id == peer.id);

                                registry.update(peer.clone());

                                println!();
                                println!("========== ONLINE PEERS ==========");
                                println!("Count: {}", registry.count());

                                for peer in registry.peers() {
                                    println!(
                                        "• {} | {} | {}",
                                        peer.name,
                                        peer.address,
                                        peer.last_seen
                                    );
                                }

                                println!("==================================");
                                println!();

                                // Connect only the first time
                                if first_time {
                                    let ip = peer.address.ip().to_string();

                                    println!(
                                        "Attempting TCP connection to {}...",
                                        ip
                                    );

                                    if let Err(err) =
                                        TransportClient::connect(&ip, 47001)
                                    {
                                        println!(
                                            "TCP connection failed: {}",
                                            err
                                        );
                                    }
                                }
                            }

                            Err(err) => {
                                println!("Invalid discovery packet: {}", err);
                            }
                        }
                    }

                    Err(err) => {
                        println!("Receive error: {}", err);
                    }
                }
            }
        });
    }
}