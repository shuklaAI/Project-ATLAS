use super::broadcaster::Broadcaster;

pub struct DiscoveryService;

impl DiscoveryService {
    pub fn start(device_id: String, device_name: String) {
        Broadcaster::start(device_id, device_name);

        // Listener will be started here next.
    }
}