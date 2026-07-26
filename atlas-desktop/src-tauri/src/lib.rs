use atlas_sdk::discovery::DiscoveryService;
use atlas_sdk::identity::get_device_id as load_device_id;
use atlas_sdk::scanner::NetworkScanner;
use atlas_sdk::transport::TransportServer;

#[tauri::command]
fn get_device_id() -> String {
    load_device_id()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ===============================
    // Atlas LAN Scanner
    // ===============================
    NetworkScanner::scan();

    let device_id = load_device_id();

    // Start TCP transport server
    TransportServer::start(47001)
        .expect("Failed to start transport server");

    // Start UDP discovery
    DiscoveryService::start(
        device_id,
        "Ghost-PC".to_string(),
    );

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}