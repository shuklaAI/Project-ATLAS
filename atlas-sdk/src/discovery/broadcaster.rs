use super::DiscoveryPacket;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

pub struct Broadcaster;

impl Broadcaster {
    pub fn start(device_id: String, device_name: String) {
        thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:0")
                .expect("Failed to bind UDP socket");

            socket
                .set_broadcast(true)
                .expect("Failed to enable broadcast");

            loop {
                let packet = DiscoveryPacket {
                    id: device_id.clone(),
                    name: device_name.clone(),
                    version: "0.1".to_string(),
                };

                let json = serde_json::to_string(&packet).unwrap();

                let _ = socket.send_to(
                    json.as_bytes(),
                    "255.255.255.255:47000",
                );

                println!("Broadcast: {}", json);

                thread::sleep(Duration::from_secs(2));
            }
        });
    }
}