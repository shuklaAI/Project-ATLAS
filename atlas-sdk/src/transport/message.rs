#[derive(Debug)]
pub enum Message {
    Handshake {
        device_id: String,
        device_name: String,
    },

    Ping,

    Pong,
}