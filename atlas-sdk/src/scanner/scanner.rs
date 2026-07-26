use super::subnet::{detect_active_network, enumerate_hosts};

pub struct NetworkScanner;

impl NetworkScanner {
    pub fn scan() {
        println!();
        println!("=================================");
        println!("Atlas LAN Scanner");
        println!("=================================");

        let network = match detect_active_network() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to detect active network: {e}");
                return;
            }
        };

        println!("Interface : {}", network.interface_name);
        println!("Local IP  : {}", network.local_ip);
        println!("Subnet    : {}/{}", network.network_addr, network.prefix_len);

        if let Some(gateway) = network.gateway {
            println!("Gateway   : {}", gateway);
        }

        println!();

        let hosts = match enumerate_hosts(&network) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Failed to enumerate hosts: {e}");
                return;
            }
        };

        println!("Generated {} hosts", hosts.len());
        println!();

        for host in hosts.iter().take(10) {
            println!("{host}");
        }

        if hosts.len() > 10 {
            println!("...");
        }
    }
}