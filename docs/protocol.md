# Atlas Transport Protocol (ATP)

Version: 1.0 (Draft)

---

# Overview

Atlas Transport Protocol (ATP) defines how Atlas clients communicate.

Every message exchanged between devices must follow ATP.

ATP is platform-independent.

Any language or operating system can implement ATP.

---

# Goals

- Platform independent
- Versioned
- Secure
- Extensible
- Low latency
- Local-first
- Cloud compatible

---

# Message Structure

Every ATP message contains the following fields.

{
    "version": 1,
    "id": "uuid",
    "timestamp": 1753393000,
    "type": "...",
    "source": "...",
    "destination": "...",
    "payload": { }
}

---

# Fields

version

Protocol version.

---

id

Unique message identifier.

UUID v4.

---

timestamp

Unix timestamp.

Used for synchronization and ordering.

---

type

Defines message purpose.

Examples

DISCOVER

PAIR_REQUEST

PAIR_ACCEPT

HEARTBEAT

CLIPBOARD_SYNC

FILE_TRANSFER

BATTERY_STATUS

NOTIFICATION_SYNC

COMMAND

PING

PONG

ERROR

---

source

Unique Atlas Device ID.

---

destination

Destination Atlas Device ID.

Can also be

BROADCAST

---

payload

Feature-specific data.

---

# Message Lifecycle

Client

↓

Serialize ATP

↓

Encrypt

↓

Transmit

↓

Receive

↓

Decrypt

↓

Deserialize

↓

Handle Message

---

# Discovery

Clients periodically broadcast

DISCOVER

Nearby Atlas devices respond with

DISCOVER_RESPONSE

---

# Pairing

PAIR_REQUEST

↓

QR Verification

↓

Key Exchange

↓

PAIR_ACCEPT

↓

Trusted Devices

---

# Sessions

Every trusted device maintains

Session ID

Encryption Keys

Heartbeat Timer

---

# Heartbeat

Every device periodically sends

HEARTBEAT

to detect offline devices.

---

# Errors

Errors use

ERROR

message.

Payload

Code

Reason

Timestamp

---

# Versioning

Future ATP versions must remain backward compatible whenever possible.

Breaking changes require

Protocol Version Increment.

---

# Future Encoding

Development

JSON

Production

MessagePack

The protocol remains identical.

Only serialization changes.

---

# Future Message Types

MEDIA_CONTROL

DEVICE_STATUS

APP_LAUNCH

AI_COMMAND

LOCATION_SHARE

PHONE_CALL

SCREEN_SHARE

CAMERA_STREAM

VOICE_STREAM

OTA_UPDATE

SMART_HOME

VEHICLE

---

End of ATP Draft v1.