# Project Atlas Architecture

## Overview

Project Atlas follows a modular, distributed architecture.

Each device runs an Atlas Client.

Clients communicate securely using the Atlas Protocol (ATP).

Cloud services are optional and primarily provide relay, synchronization, authentication, and backup.

Whenever possible, communication occurs directly between trusted devices.

---

# High-Level Architecture

                   Atlas Cloud
      Authentication • Relay • Sync • API
                        │
                Encrypted WebSocket
                        │
 ┌──────────────────────┼──────────────────────┐
 │                      │                      │
 │                      │                      │
Linux Client      Android Client       Windows Client
 │                      │                      │
 └────────────── Local Network ───────────────┘

---

# Components

## Atlas Desktop

Responsibilities

- Device discovery
- Clipboard synchronization
- File transfer
- Notification display
- Local automation
- Device management

---

## Atlas Android

Responsibilities

- Notifications
- Clipboard
- Device pairing
- Battery information
- Phone calls
- Hotspot control
- Background synchronization

---

## Atlas Server

Responsibilities

- Authentication
- Device registry
- Relay server
- Push synchronization
- Offline message delivery
- API

The server should never require access to plaintext user data.

---

## Atlas SDK

Provides

- ATP implementation
- Encryption
- Device discovery
- Pairing
- Session management
- Common utilities

Every Atlas application should use the SDK instead of implementing networking independently.

---

# Communication Layers

Priority order

1. Local Network
2. Bluetooth (future)
3. Wi-Fi Direct (future)
4. Internet Relay

Atlas always prefers the fastest available direct connection.

---

# Communication Flow

Example

Device A

↓

Discovers Device B

↓

Pairing

↓

Exchange Keys

↓

Encrypted Session

↓

ATP Messages

↓

Synchronization

---

# Design Principles

Local First

Devices should communicate directly whenever possible.

---

Cloud Optional

Cloud should extend the experience, not be required.

---

Platform Independent

Every operating system should implement the same protocol.

---

Security First

All communication must be encrypted.

---

Extensible

Adding a new feature should require only introducing a new ATP message type.

---

# Future Architecture

Phase 1

Linux
Android

Phase 2

Windows

Phase 3

macOS

Phase 4

Wearables

Phase 5

IoT

Phase 6

Automotive

---

# Long-Term Vision

Atlas becomes the interoperability platform connecting every personal device regardless of manufacturer.