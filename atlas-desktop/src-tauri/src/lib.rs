use atlas_sdk::identity::get_device_id as load_device_id;
use atlas_sdk::discovery::DiscoveryService;

#[tauri::command]
fn get_device_id() -> String {
    load_device_id()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let device_id = load_device_id();

    DiscoveryService::start(
        device_id,
        "Ghost-PC".to_string(),
    );

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}