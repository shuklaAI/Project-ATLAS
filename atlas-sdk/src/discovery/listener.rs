use super::broadcaster::DISCOVERY_PORT;
use super::{DiscoveryPacket, Peer, PeerRegistry};

use crate::transport::TransportClient;

use socket2::{Domain, Protocol, Socket, Type};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Listener;

impl Listener {
    pub fn start(local_device_id: String, registry: Arc<Mutex<PeerRegistry>>) {
        thread::spawn(move || {
            let socket = match Self::build_socket() {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("[listener] failed to bind UDP {DISCOVERY_PORT}: {err}");
                    return;
                }
            };

            println!("Discovery listener started on UDP {DISCOVERY_PORT}");

            let mut buffer = [0u8; 4096];

            loop {
                match socket.recv_from(&mut buffer) {
                    Ok((size, addr)) => {
                        Self::handle_packet(&buffer[..size], addr, &local_device_id, &registry);
                    }
                    Err(err) => eprintln!("[listener] receive error: {err}"),
                }
            }
        });
    }

    fn build_socket() -> std::io::Result<UdpSocket> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        socket.set_reuse_address(true)?;
        #[cfg(unix)]
        socket.set_reuse_port(true)?;
        socket.bind(&SocketAddr::from(([0, 0, 0, 0], DISCOVERY_PORT)).into())?;
        Ok(socket.into())
    }

    fn handle_packet(
        data: &[u8],
        addr: SocketAddr,
        local_device_id: &str,
        registry: &Arc<Mutex<PeerRegistry>>,
    ) {
        let text = String::from_utf8_lossy(data);

        let packet: DiscoveryPacket = match serde_json::from_str(&text) {
            Ok(p) => p,
            Err(err) => {
                eprintln!("[listener] invalid discovery packet from {addr}: {err}");
                return;
            }
        };

        if packet.id == local_device_id {
            return; // our own broadcast
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

        let first_time = {
            let mut registry = registry.lock().unwrap();
            let first_time = !registry.peers().iter().any(|p| p.id == peer.id);
            registry.update(peer.clone());
            Self::print_peers(&registry);
            first_time
        };

        // Connect on its own thread: a slow/unreachable peer must never
        // block this UDP loop from processing everyone else's broadcasts.
        if first_time {
            let ip = peer.address.ip().to_string();
            thread::spawn(move || {
                println!("Attempting TCP connection to {ip}...");
                if let Err(err) = TransportClient::connect(&ip, 47001) {
                    println!("TCP connection to {ip} failed: {err}");
                }
            });
        }
    }

    fn print_peers(registry: &PeerRegistry) {
        println!("\n========== ONLINE PEERS ==========");
        println!("Count: {}", registry.count());
        for peer in registry.peers() {
            println!("• {} | {} | {}", peer.name, peer.address, peer.last_seen);
        }
        println!("==================================\n");
    }
}