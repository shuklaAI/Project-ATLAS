use std::fs;
use std::path::PathBuf;

use uuid::Uuid;

fn device_file() -> PathBuf {
    let mut dir = dirs::data_local_dir().expect("Could not determine local data directory");

    dir.push("atlas");

    fs::create_dir_all(&dir).expect("Could not create atlas directory");

    dir.push("device_id");

    dir
}

pub fn get_device_id() -> String {
    let file = device_file();

    if file.exists() {
        fs::read_to_string(file)
            .expect("Could not read device ID")
            .trim()
            .to_string()
    } else {
        let id = Uuid::new_v4().to_string();

        fs::write(&file, &id)
            .expect("Could not save device ID");

        id
    }
}