//! Active network interface and subnet detection.
//!
//! This module answers two questions that every other part of the scanner
//! depends on:
//!
//! 1. Which network interface is actually being used to reach the rest of
//!    the world (or the LAN) right now?
//! 2. What IPv4 subnet is that interface on, and which host addresses does
//!    that subnet contain?
//!
//! Detection is cross-platform (Linux, Windows, macOS) via the `default-net`
//! crate, which queries the OS routing table / interface list natively on
//! each platform instead of shelling out to `ip`, `ifconfig`, or
//! `ipconfig`. Host enumeration is done with `ipnet`, which handles CIDR
//! math correctly (including edge cases like /31 and /32).
//!
//! # Cargo.toml
//! ```toml
//! [dependencies]
//! default-net = "0.22"
//! ipnet = "2"
//! thiserror = "1"
//! ```

use std::net::Ipv4Addr;

use ipnet::Ipv4Net;
use thiserror::Error;

/// Errors that can occur while detecting the active network or expanding
/// it into a host list.
#[derive(Debug, Error)]
pub enum SubnetError {
    #[error("failed to query default network interface: {0}")]
    InterfaceDetection(String),

    #[error("the default interface has no IPv4 address assigned")]
    NoIpv4Address,

    #[error("invalid network prefix length: {0}")]
    InvalidPrefix(u8),

    #[error("subnet /{0} is too large to scan ({1} hosts); refusing to enumerate")]
    SubnetTooLarge(u8, u64),
}

/// The maximum number of host addresses we are willing to enumerate for a
/// single scan. This protects callers from accidentally trying to scan,
/// say, a misconfigured /8. A /16 (65,534 hosts) is already a lot for a
/// LAN discovery tool and is allowed; anything larger is rejected.
const MAX_SCANNABLE_HOSTS: u64 = 65_534;

/// Everything the scanner needs to know about the network it's running on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInfo {
    /// OS-level interface name, e.g. `"eth0"`, `"en0"`, `"Ethernet"`.
    pub interface_name: String,

    /// The local machine's IPv4 address on this interface.
    pub local_ip: Ipv4Addr,

    /// CIDR prefix length, e.g. `24` for a /24.
    pub prefix_len: u8,

    /// Subnet mask derived from `prefix_len`, e.g. `255.255.255.0`.
    pub subnet_mask: Ipv4Addr,

    /// Network address of the subnet, e.g. `192.168.1.0`.
    pub network_addr: Ipv4Addr,

    /// Broadcast address of the subnet, e.g. `192.168.1.255`.
    pub broadcast_addr: Ipv4Addr,

    /// Default gateway, if one could be determined.
    pub gateway: Option<Ipv4Addr>,
}

impl NetworkInfo {
    /// Total number of usable host addresses in this subnet, excluding the
    /// network and broadcast addresses (for prefixes < 31).
    pub fn host_count(&self) -> u64 {
        let net: Ipv4Net = match Ipv4Net::new(self.network_addr, self.prefix_len) {
            Ok(n) => n,
            Err(_) => return 0,
        };
        usable_host_count(&net)
    }

    /// Returns true if `ip` falls within this subnet.
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        match Ipv4Net::new(self.network_addr, self.prefix_len) {
            Ok(net) => net.contains(&ip),
            Err(_) => false,
        }
    }
}

/// Detects the network interface currently used for outbound traffic and
/// returns full subnet information for it.
///
/// This relies on the OS's own notion of the "default" interface (the one
/// tied to the default route), which is the correct way to determine
/// "the network the user is on" across Linux, Windows, and macOS, VPNs,
/// Wi-Fi/Ethernet priority, and multi-homed hosts included.
///
/// # Errors
/// Returns [`SubnetError::InterfaceDetection`] if no default interface can
/// be found (e.g. the machine is offline), or
/// [`SubnetError::NoIpv4Address`] if the default interface has no IPv4
/// address (IPv6-only networks are not yet supported by the scanner).
pub fn detect_active_network() -> Result<NetworkInfo, SubnetError> {
    let default_iface = default_net::get_default_interface()
        .map_err(|e| SubnetError::InterfaceDetection(e.to_string()))?;

    let ipv4 = default_iface
        .ipv4
        .first()
        .ok_or(SubnetError::NoIpv4Address)?;

    let local_ip = ipv4.addr;
    let prefix_len = ipv4.prefix_len;

    let net = Ipv4Net::new(local_ip, prefix_len)
        .map_err(|_| SubnetError::InvalidPrefix(prefix_len))?;

    let gateway = default_iface
        .gateway
        .as_ref()
        .and_then(|gw| match gw.ip_addr {
            std::net::IpAddr::V4(v4) => Some(v4),
            std::net::IpAddr::V6(_) => None,
        });

    Ok(NetworkInfo {
        interface_name: default_iface.name,
        local_ip,
        prefix_len,
        subnet_mask: net.netmask(),
        network_addr: net.network(),
        broadcast_addr: net.broadcast(),
        gateway,
    })
}

/// Expands a [`NetworkInfo`] into the full list of host addresses to scan,
/// excluding the network address and broadcast address (for prefixes with
/// a distinct network/broadcast, i.e. < /31).
///
/// The local machine's own address is intentionally *included*; callers
/// that want to skip self-probing can filter it out using
/// `network_info.local_ip`.
///
/// # Errors
/// Returns [`SubnetError::SubnetTooLarge`] if the subnet contains more
/// than [`MAX_SCANNABLE_HOSTS`] addresses, to avoid accidentally launching
/// a scan against tens of thousands of hosts due to a misconfigured
/// interface (e.g. a /8 assigned by a broken DHCP server).
pub fn enumerate_hosts(network_info: &NetworkInfo) -> Result<Vec<Ipv4Addr>, SubnetError> {
    let net = Ipv4Net::new(network_info.network_addr, network_info.prefix_len)
        .map_err(|_| SubnetError::InvalidPrefix(network_info.prefix_len))?;

    let count = usable_host_count(&net);
    if count > MAX_SCANNABLE_HOSTS {
        return Err(SubnetError::SubnetTooLarge(network_info.prefix_len, count));
    }

    let hosts = match network_info.prefix_len {
        // /31 and /32: no distinct network/broadcast address, every
        // address in the range is a usable host (RFC 3021 for /31).
        31 | 32 => net.hosts().collect(),
        _ => net
            .hosts()
            .filter(|ip| *ip != net.network() && *ip != net.broadcast())
            .collect(),
    };

    Ok(hosts)
}

/// Number of usable host addresses in a network, matching the semantics
/// used by [`enumerate_hosts`] (network/broadcast excluded for prefixes
/// below /31).
fn usable_host_count(net: &Ipv4Net) -> u64 {
    let prefix = net.prefix_len();
    if prefix >= 32 {
        return 1;
    }
    if prefix == 31 {
        return 2;
    }
    let host_bits = 32 - prefix as u32;
    (1u64 << host_bits) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(local_ip: &str, prefix_len: u8) -> NetworkInfo {
        let local_ip: Ipv4Addr = local_ip.parse().unwrap();
        let net = Ipv4Net::new(local_ip, prefix_len).unwrap();
        NetworkInfo {
            interface_name: "test0".to_string(),
            local_ip,
            prefix_len,
            subnet_mask: net.netmask(),
            network_addr: net.network(),
            broadcast_addr: net.broadcast(),
            gateway: None,
        }
    }

    #[test]
    fn enumerates_standard_slash_24() {
        let ni = info("192.168.1.42", 24);
        let hosts = enumerate_hosts(&ni).unwrap();

        assert_eq!(hosts.len(), 254);
        assert!(!hosts.contains(&"192.168.1.0".parse().unwrap()));
        assert!(!hosts.contains(&"192.168.1.255".parse().unwrap()));
        assert!(hosts.contains(&"192.168.1.1".parse().unwrap()));
        assert!(hosts.contains(&"192.168.1.254".parse().unwrap()));
    }

    #[test]
    fn enumerates_small_slash_29() {
        let ni = info("10.0.0.10", 29);
        let hosts = enumerate_hosts(&ni).unwrap();
        // /29 = 8 addresses, minus network+broadcast = 6 usable hosts.
        assert_eq!(hosts.len(), 6);
    }

    #[test]
    fn handles_point_to_point_slash_31() {
        let ni = info("10.0.0.0", 31);
        let hosts = enumerate_hosts(&ni).unwrap();
        assert_eq!(hosts.len(), 2);
    }

    #[test]
    fn rejects_oversized_subnet() {
        let ni = info("10.0.0.1", 8);
        let err = enumerate_hosts(&ni).unwrap_err();
        assert!(matches!(err, SubnetError::SubnetTooLarge(8, _)));
    }

    #[test]
    fn contains_checks_membership_correctly() {
        let ni = info("192.168.1.42", 24);
        assert!(ni.contains("192.168.1.200".parse().unwrap()));
        assert!(!ni.contains("192.168.2.1".parse().unwrap()));
    }

    #[test]
    fn host_count_matches_enumeration_length() {
        let ni = info("172.16.5.5", 26);
        let hosts = enumerate_hosts(&ni).unwrap();
        assert_eq!(hosts.len() as u64, ni.host_count());
    }
}