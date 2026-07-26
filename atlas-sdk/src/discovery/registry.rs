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
}