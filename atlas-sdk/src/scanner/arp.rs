use std::collections::HashMap;
use std::net::IpAddr;

/// Snapshot of the OS ARP/neighbor cache. Call this AFTER probing hosts —
/// the OS only populates ARP entries for addresses it just had to resolve
/// at layer 2, which happens automatically the moment we TCP-connect to
/// something on the local subnet.
pub fn read_arp_table() -> HashMap<IpAddr, String> {
    #[cfg(target_os = "linux")]
    {
        read_proc_net_arp()
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        read_via_arp_command()
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        HashMap::new()
    }
}

#[cfg(target_os = "linux")]
fn read_proc_net_arp() -> HashMap<IpAddr, String> {
    use std::fs;
    let mut table = HashMap::new();

    let contents = match fs::read_to_string("/proc/net/arp") {
        Ok(c) => c,
        Err(_) => return table,
    };

    for line in contents.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 4 {
            continue;
        }
        let Ok(ip) = cols[0].parse::<IpAddr>() else {
            continue;
        };
        let mac = cols[3];
        if mac == "00:00:00:00:00:00" {
            continue;
        }
        table.insert(ip, normalize_mac(mac));
    }

    table
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn read_via_arp_command() -> HashMap<IpAddr, String> {
    use std::process::Command;

    let mut table = HashMap::new();

    let Ok(output) = Command::new("arp").arg("-a").output() else {
        return table;
    };

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if let Some((ip, mac)) = parse_arp_line(line) {
            table.insert(ip, mac);
        }
    }

    table
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn parse_arp_line(line: &str) -> Option<(IpAddr, String)> {
    // macOS: "hostname (192.168.1.1) at aa:bb:cc:dd:ee:ff on en0 ..."
    if let Some(start) = line.find('(') {
        if let Some(end) = line.find(')') {
            let ip: IpAddr = line[start + 1..end].parse().ok()?;
            let mac = line.split("at ").nth(1)?.split_whitespace().next()?;
            return Some((ip, normalize_mac(mac)));
        }
    }

    // Windows: "  192.168.1.1          aa-bb-cc-dd-ee-ff     dynamic"
    let cols: Vec<&str> = line.split_whitespace().collect();
    if cols.len() >= 2 {
        if let Ok(ip) = cols[0].parse::<IpAddr>() {
            if cols[1].contains('-') || cols[1].contains(':') {
                return Some((ip, normalize_mac(cols[1])));
            }
        }
    }

    None
}

fn normalize_mac(mac: &str) -> String {
    mac.replace('-', ":").to_lowercase()
}

/// Small built-in OUI table for common vendors. Not exhaustive — swap for
/// the full IEEE OUI CSV later if you need broader coverage.
pub fn lookup_vendor(mac: &str) -> Option<String> {
    let oui = mac.get(0..8)?.to_uppercase();
    let vendor = match oui.as_str() {
        "A4:77:33" | "AC:BC:32" | "F0:18:98" | "00:1B:63" | "00:1E:C2" => "Apple",
        "00:1A:11" | "3C:5A:B4" | "F4:F5:D8" => "Google",
        "B8:27:EB" | "DC:A6:32" | "E4:5F:01" => "Raspberry Pi Foundation",
        "00:50:56" | "00:0C:29" => "VMware",
        "08:00:27" => "VirtualBox",
        "FC:EC:DA" | "74:C2:46" => "Amazon Technologies",
        "00:17:88" => "Philips (Hue)",
        "B0:BE:76" | "5C:0A:5B" | "18:D6:C7" => "Samsung",
        "F8:1A:67" | "50:C7:BF" | "94:83:C4" => "TP-Link",
        _ => return None,
    };
    Some(vendor.to_string())
}