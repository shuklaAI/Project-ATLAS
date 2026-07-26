use std::net::IpAddr;

#[derive(Clone, Debug)]
pub struct Device {
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub mac: Option<String>,
    pub vendor: Option<String>,
    pub online: bool,
    pub atlas: bool,
    pub latency_ms: Option<u128>,
}