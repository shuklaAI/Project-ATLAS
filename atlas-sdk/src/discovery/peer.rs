use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub name: String,
    pub address: String,
    pub last_seen: u64,
}