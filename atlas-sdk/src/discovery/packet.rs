use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryPacket {
    pub id: String,
    pub name: String,
    pub version: String,
}