use atlas_sdk::discovery::{DiscoveryService, Peer, PeerRegistry};
use atlas_sdk::identity::get_device_id as load_device_id;
use atlas_sdk::scanner::NetworkScanner;
use atlas_sdk::transport::TransportServer;
use std::sync::{Arc, Mutex};
use tauri::State;

struct AppState {
    registry: Arc<Mutex<PeerRegistry>>,
}

#[tauri::command]
fn get_device_id() -> String {
    load_device_id()
}

#[tauri::command]
fn get_online_peers(state: State<AppState>) -> Vec<Peer> {
    state
        .registry
        .lock()
        .unwrap()
        .peers()
        .into_iter()
        .cloned()
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load persistent device ID
    let device_id = load_device_id();

    // Start Atlas transport server FIRST so this machine can be detected.
    TransportServer::start(47001)
        .expect("Failed to start transport server");

    // Start discovery (broadcast + listener)
    let registry = DiscoveryService::start(
        device_id,
        "Ghost-PC".to_string(),
    );

    // Scan the LAN after Atlas services are online
    NetworkScanner::scan();

    tauri::Builder::default()
        .manage(AppState { registry })
        .invoke_handler(tauri::generate_handler![
            get_device_id,
            get_online_peers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}