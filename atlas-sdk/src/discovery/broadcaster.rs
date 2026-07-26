use super::DiscoveryPacket;
use crate::scanner::subnet;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub const DISCOVERY_PORT: u16 = 47000;

pub struct Broadcaster;

impl Broadcaster {
    /// Starts broadcasting this device's presence every 2 seconds.
    /// Returns a handle you can flip to `false` to stop the thread cleanly.
    pub fn start(device_id: String, device_name: String) -> Arc<AtomicBool> {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        thread::spawn(move || {
            let socket = match Self::build_socket() {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("[broadcaster] failed to create socket: {err}");
                    return;
                }
            };

            while running_clone.load(Ordering::Relaxed) {
                let packet = DiscoveryPacket {
                    id: device_id.clone(),
                    name: device_name.clone(),
                    version: "0.1".to_string(),
                };

                match serde_json::to_string(&packet) {
                    Ok(json) => {
                        for target in Self::broadcast_targets() {
                            if let Err(err) = socket.send_to(json.as_bytes(), target) {
                                eprintln!("[broadcaster] send to {target} failed: {err}");
                            }
                        }
                    }
                    Err(err) => eprintln!("[broadcaster] failed to encode packet: {err}"),
                }

                thread::sleep(Duration::from_secs(2));
            }
        });

        running
    }

    /// SO_REUSEADDR/SO_REUSEPORT so a restarted process (or a second Atlas
    /// instance on the same host, e.g. during testing) doesn't fail to bind.
    fn build_socket() -> std::io::Result<UdpSocket> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        socket.set_reuse_address(true)?;
        #[cfg(unix)]
        socket.set_reuse_port(true)?;
        socket.set_broadcast(true)?;
        socket.bind(&SocketAddr::from(([0, 0, 0, 0], 0)).into())?;
        Ok(socket.into())
    }

    /// Send to both the subnet-directed broadcast (e.g. 192.168.1.255,
    /// which most routers forward even when they block global broadcast)
    /// AND 255.255.255.255 as a fallback. Belt and suspenders.
    fn broadcast_targets() -> Vec<SocketAddr> {
        let mut targets = vec![SocketAddr::from(([255, 255, 255, 255], DISCOVERY_PORT))];

        if let Ok(net) = subnet::detect_active_network() {
            targets.push(SocketAddr::new(net.broadcast_addr.into(), DISCOVERY_PORT));
        }

        targets
    }
}