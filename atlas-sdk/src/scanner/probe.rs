use crate::scanner::device::Device;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Common ports to probe when checking whether a host is alive. We don't
/// care whether any of these are actually "open" for our purposes — a
/// connection attempt that gets actively refused proves the host exists
/// just as well as one that succeeds. Only a timeout means "no host here".
const PROBE_PORTS: &[u16] = &[80, 443, 22, 445, 139, 53, 8080];

/// The Atlas node port from the architecture spec.
pub const ATLAS_PORT: u16 = 47001;

const CONNECT_TIMEOUT: Duration = Duration::from_millis(400);

/// Max hosts probed concurrently. This is a worker pool bound via a
/// semaphore — NOT one OS thread per IP.
const MAX_CONCURRENT_PROBES: usize = 128;

/// Probes every host concurrently and returns a `Device` for each one
/// that responded on any checked port.
pub async fn probe_hosts(hosts: Vec<IpAddr>) -> Vec<Device> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_PROBES));
    let mut tasks = Vec::with_capacity(hosts.len());

    for ip in hosts {
        let semaphore = semaphore.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = semaphore
                .acquire_owned()
                .await
                .expect("semaphore closed");

            probe_host(ip).await
        }));
    }

    let mut devices = Vec::with_capacity(tasks.len());

    for task in tasks {
        if let Ok(Some(device)) = task.await {
            devices.push(device);
        }
    }

    devices
}

async fn probe_host(ip: IpAddr) -> Option<Device> {
    let start = Instant::now();

    // Check the Atlas port first.
    let atlas = tcp_responds(ip, ATLAS_PORT).await;

    let online = atlas || is_online(ip).await;

    if !online {
        return None;
    }

    let latency_ms = Some(start.elapsed().as_millis());
    let hostname = resolve_hostname(ip).await;

    Some(Device {
        ip,
        hostname,
        mac: None, // filled in later from ARP table
        vendor: None,
        online,
        atlas,
        latency_ms,
    })
}

/// True if the host responded on any common port.
async fn is_online(ip: IpAddr) -> bool {
    let checks = PROBE_PORTS
        .iter()
        .map(|&port| tcp_responds(ip, port));

    futures::future::join_all(checks)
        .await
        .into_iter()
        .any(|r| r)
}

/// Returns true ONLY if a TCP connection is successfully established.
///
/// Timeouts, connection refused, unreachable hosts, etc. all return false.
async fn tcp_responds(ip: IpAddr, port: u16) -> bool {
    let addr = SocketAddr::new(ip, port);

    match timeout(CONNECT_TIMEOUT, TcpStream::connect(addr)).await {
        Ok(Ok(_stream)) => true,
        Ok(Err(_)) => false,
        Err(_) => false,
    }
}

/// Reverse DNS lookup.
async fn resolve_hostname(ip: IpAddr) -> Option<String> {
    tokio::task::spawn_blocking(move || dns_lookup::lookup_addr(&ip).ok())
        .await
        .ok()
        .flatten()
        .filter(|name| !name.is_empty() && name.parse::<IpAddr>().is_err())
}