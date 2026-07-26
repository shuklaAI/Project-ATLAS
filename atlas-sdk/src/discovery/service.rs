use super::broadcaster::Broadcaster;
use super::listener::Listener;
use super::PeerRegistry;

use std::sync::{Arc, Mutex};

pub struct DiscoveryService;

impl DiscoveryService {
    pub fn start(device_id: String, device_name: String) {
        let registry = Arc::new(Mutex::new(PeerRegistry::new()));

        Broadcaster::start(device_id.clone(), device_name);
        Listener::start(device_id, registry);
    }
}