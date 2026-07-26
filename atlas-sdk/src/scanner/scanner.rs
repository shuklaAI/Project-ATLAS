use super::arp;
use super::device::Device;
use super::probe;
use super::subnet::{detect_active_network, enumerate_hosts};
use std::net::IpAddr;

pub struct NetworkScanner;

impl NetworkScanner {
    /// Sync wrapper for callers not already inside a tokio runtime (e.g.
    /// Tauri's `run()`). Spins up a short-lived runtime and blocks on it.
    pub fn scan() -> Vec<Device> {
        match tokio::runtime::Runtime::new() {
            Ok(rt) => rt.block_on(Self::scan_async()),
            Err(err) => {
                eprintln!("Failed to start scanner runtime: {err}");
                Vec::new()
            }
        }
    }

    pub async fn scan_async() -> Vec<Device> {
        println!("\n=================================");
        println!("Atlas LAN Scanner");
        println!("=================================");

        let network = match detect_active_network() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to detect active network: {e}");
                return Vec::new();
            }
        };

        println!("Interface : {}", network.interface_name);
        println!("Local IP  : {}", network.local_ip);
        println!("Subnet    : {}/{}", network.network_addr, network.prefix_len);
        if let Some(gateway) = network.gateway {
            println!("Gateway   : {}", gateway);
        }

        let hosts = match enumerate_hosts(&network) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Failed to enumerate hosts: {e}");
                return Vec::new();
            }
        };

        println!("\nProbing {} hosts...", hosts.len());

        let ip_hosts: Vec<IpAddr> = hosts.into_iter().map(IpAddr::V4).collect();
        let mut devices = probe::probe_hosts(ip_hosts).await;

        // ARP cache is populated now that we've reached these hosts.
        let arp_table = arp::read_arp_table();
        for device in &mut devices {
            if let Some(mac) = arp_table.get(&device.ip) {
                device.vendor = arp::lookup_vendor(mac);
                device.mac = Some(mac.clone());
            }
        }

        devices.sort_by_key(|d| d.ip);

        println!("\nFound {} device(s) online:", devices.len());
        println!(
            "{:<16} {:<7} {:<18} {:<20} {}",
            "IP", "ATLAS", "MAC", "VENDOR", "HOSTNAME"
        );
        for d in &devices {
            println!(
                "{:<16} {:<7} {:<18} {:<20} {}",
                d.ip.to_string(),
                if d.atlas { "yes" } else { "no" },
                d.mac.as_deref().unwrap_or("-"),
                d.vendor.as_deref().unwrap_or("-"),
                d.hostname.as_deref().unwrap_or("-"),
            );
        }
        println!();

        devices
    }
}