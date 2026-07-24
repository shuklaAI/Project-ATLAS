mod device;

use device::identity::get_device_id as load_device_id;

#[tauri::command]
fn get_device_id() -> String {
    load_device_id()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_device_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}