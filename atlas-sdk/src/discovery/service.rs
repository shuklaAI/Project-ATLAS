use super::broadcaster::Broadcaster;
use super::listener::Listener;
use super::PeerRegistry;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct DiscoveryService;

impl DiscoveryService {
    /// Starts broadcasting + listening and returns the shared registry so
    /// callers (Tauri commands, Android JNI, tests) can actually read who's
    /// online — not just watch console output.
    pub fn start(device_id: String, device_name: String) -> Arc<Mutex<PeerRegistry>> {
        let registry = Arc::new(Mutex::new(PeerRegistry::new()));

        Broadcaster::start(device_id.clone(), device_name);
        Listener::start(device_id, registry.clone());

        // Broadcast interval is 2s; treat 10s (5 missed beats) as offline.
        let prune_registry = registry.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            prune_registry.lock().unwrap().prune_stale(10);
        });

        registry
    }
}