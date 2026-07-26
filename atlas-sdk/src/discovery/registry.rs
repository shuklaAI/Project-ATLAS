use std::collections::HashMap;

use super::Peer;

pub struct PeerRegistry {
    peers: HashMap<String, Peer>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn update(&mut self, peer: Peer) {
        self.peers.insert(peer.id.clone(), peer);
    }

    pub fn remove(&mut self, id: &str) {
        self.peers.remove(id);
    }

    pub fn peers(&self) -> Vec<&Peer> {
        self.peers.values().collect()
    }

    pub fn count(&self) -> usize {
        self.peers.len()
    }

    /// Drops any peer we haven't heard a broadcast from in `ttl_secs`.
    /// Without this, devices that went offline stay "online" forever.
    pub fn prune_stale(&mut self, ttl_secs: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.peers
            .retain(|_, peer| now.saturating_sub(peer.last_seen) <= ttl_secs);
    }
}