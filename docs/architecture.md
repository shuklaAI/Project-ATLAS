# Atlas Architecture Specification

Version: 0.1
Status: Active Development

---

# Project Vision

Atlas is a cross-platform peer-to-peer ecosystem designed to discover, identify, communicate with, and manage nearby devices over multiple communication mediums.

Unlike traditional applications that only communicate with other running instances, Atlas aims to build a complete networking layer capable of discovering every reachable device on a local network while automatically upgrading communication when another Atlas node is detected.

The long-term goal is to build an open-source alternative to proprietary ecosystems such as KDE Connect, AirDrop, LocalSend, Nearby Share, and Syncthing while exposing a reusable networking SDK for future Atlas applications.

Atlas is NOT intended to be a simple chat application.

Atlas is a networking platform.

---

# High-Level Architecture

```
                 +---------------------+
                 |   Atlas Desktop     |
                 +---------------------+
                           |
                 +---------------------+
                 |     atlas-sdk       |
                 +---------------------+

     ┌──────────────┬───────────────┬──────────────┐
     │              │               │              │
 Discovery      Scanner        Transport      Identity
     │              │               │              │
     └──────────────┴───────────────┴──────────────┘
                    │
             Device Manager
                    │
          Shared Device Registry
                    │
            Future Services Layer
```

The SDK contains all networking logic.

Desktop, Android and future applications consume the SDK.

---

# Workspace Layout

```
atlas/

atlas-sdk/
atlas-desktop/
atlas-android/
atlas-server/

docs/
```

---

# SDK Modules

## Identity

Responsible for:

- persistent device ID
- device name
- device metadata

Every Atlas installation has a unique identity.

---

## Discovery

Purpose:

Discover OTHER Atlas devices.

Uses:

UDP Broadcast

Default Port:

47000

Broadcast packets contain:

- device id
- device name
- protocol version

Discovery DOES NOT detect generic LAN devices.

It only discovers Atlas nodes.

---

## Scanner

Purpose:

Discover ALL devices on the local network.

This is completely different from Atlas discovery.

Responsibilities:

- detect active interface
- detect subnet
- enumerate hosts
- probe hosts
- resolve hostname
- obtain MAC address
- identify vendor
- detect Atlas capability

Scanner should eventually return

```
Vec<Device>
```

---

## Transport

Responsible for reliable communication.

Uses TCP.

Default Port:

47001

Future messages:

- handshake
- ping
- chat
- clipboard
- files
- remote terminal
- screen sharing

Transport is only used AFTER discovery.

---

# Discovery Flow

```
Atlas starts

↓

Identity loads

↓

Transport server starts

↓

UDP broadcaster starts

↓

UDP listener starts

↓

Nearby Atlas device found

↓

TCP connection established

↓

Handshake

↓

Peer Registry updated
```

---

# Scanner Flow

```
Atlas starts

↓

Detect active network interface

↓

Determine subnet

↓

Enumerate IP addresses

↓

Concurrent TCP probe

↓

Alive hosts

↓

Resolve hostname

↓

Retrieve MAC address

↓

Determine vendor

↓

Check Atlas port (47001)

↓

Return Vec<Device>
```

---

# Device Model

Every discovered device should eventually contain

```rust
pub struct Device {
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub mac: Option<String>,
    pub vendor: Option<String>,
    pub online: bool,
    pub atlas: bool,
    pub latency_ms: Option<u128>,
}
```

Additional fields may be added in future.

---

# Bluetooth

Bluetooth discovery is NOT implemented yet.

Future architecture:

```
scanner/

    lan/

    bluetooth/

    atlas/
```

Bluetooth should eventually discover:

- phones
- tablets
- laptops
- speakers
- headphones

using native platform APIs.

---

# Current Status

Implemented

✓ Identity

✓ UDP Atlas Discovery

✓ Peer Registry

✓ TCP Transport

✓ Desktop UI

✓ Active subnet detection

✓ Host enumeration

Not Yet Implemented

✗ Concurrent LAN probing

✗ Hostname resolution

✗ MAC retrieval

✗ Vendor lookup

✗ Bluetooth scanning

✗ Device manager

✗ Service layer

---

# Engineering Principles

The project should follow these rules.

1.

Cross-platform first.

Support:

- Linux
- Windows
- macOS
- Android

Avoid platform-specific shell commands whenever possible.

Use native Rust libraries.

---

2.

Do not duplicate networking logic.

All networking belongs inside atlas-sdk.

Desktop and Android should remain thin clients.

---

3.

Production quality.

Avoid toy implementations.

Avoid blocking code where asynchronous code is appropriate.

Prefer Tokio for networking.

---

4.

Modular design.

Small focused modules are preferred over large files.

---

5.

Strong typing.

Avoid String-based error handling.

Use custom error enums.

---

6.

Concurrency.

Large LAN scans should use worker pools.

Do NOT spawn unlimited threads.

---

7.

Extensibility.

Future networking protocols should integrate without major rewrites.

---

# Current Roadmap

Phase 1

✓ Identity

✓ Discovery

✓ Transport

✓ Desktop

Phase 2

LAN Scanner

- subnet detection
- concurrent probing
- hostname
- MAC
- vendor
- Atlas detection

Phase 3

Bluetooth Discovery

Phase 4

Device Manager

Phase 5

Services

- Chat
- Clipboard
- File Transfer
- Remote Terminal
- Screen Sharing

Phase 6

Android Integration

---

# Guidance For AI Assistants

When modifying Atlas:

- Preserve the modular architecture.
- Do not replace the networking architecture without justification.
- Prefer extending existing modules over rewriting them.
- Keep networking logic inside atlas-sdk.
- Maintain cross-platform compatibility.
- Use production-quality Rust.
- Avoid placeholder or demonstration code unless explicitly requested.
- When implementing new functionality, ensure it integrates cleanly with existing modules and public APIs.
- If introducing a new dependency, explain why it is needed and prefer mature, actively maintained crates.