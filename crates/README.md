# Code Architecture

This directory contains all the Rust code for the Clef project.

The codebase is organized as a **Rust workspace**, split into multiple crates with
**strict responsibilities and dependency boundaries**.

The goal of this structure is to:

- keep cryptographic logic isolated and auditable
- make the protocol rules explicit and testable
- avoid coupling UI, transport, and security logic
- allow reuse across client, server, and future targets (WASM, mobile, etc.)

---

## Crate Overview

### `crypto`

**Lowest-level crate. Security foundation.**

This crate implements all cryptographic primitives and key management logic used by the project.

It includes:

- key generation (identity, device, symmetric keys)
- encryption and decryption
- key derivation (epochs, root keys, subkeys)
- strongly typed cryptographic states and identifiers

**This crate has no knowledge of:**

- networking
- HTTP / WebSockets
- serialization formats
- clients or servers

It must remain deterministic, testable, and independent of the outside world.

---

### `protocol`

**Protocol rules and state machine.**

This crate defines:

- protocol-level data structures
- message types and validation rules
- client / server state transitions
- invariants enforced by the protocol

It uses the `crypto` crate to enforce security guarantees,
but does not deal with how messages are transported.

**This crate does NOT handle:**

- HTTP or WebSockets
- storage backends
- UI concerns

The protocol should be fully testable without any network.

---

### `client_core`

**Client-side logic without UI.**

This crate implements the core client behavior:

- local client state management
- interaction with the protocol layer
- orchestration of synchronization and epochs
- abstraction over networking and storage

It is designed to be reusable by multiple frontends
(CLI, GUI, web, mobile).

This crate does not perform rendering or user interaction directly.

---

### `client`

**Client executable / UI layer.**

This crate is an executable that:

- provides a user interface (CLI, GUI, etc.)
- translates user actions into `client_core` calls
- displays state and results to the user

It contains no protocol or cryptographic logic.
All security-critical behavior lives below this layer.

---

### `server`

**Server-side logic.**

This crate implements the instance / server behavior:

- handling client requests
- validating protocol messages
- coordinating synchronization between clients
- storing and serving encrypted blobs

The server does not have access to plaintext user data
and relies on the same `crypto` and `protocol` crates
to enforce correctness.

---

## Dependency Rules

Dependencies must flow strictly in one direction:

```
crypto
↓
protocol
↓
client_core / server
↓
client
```

Reverse dependencies are not allowed.

---

## Design Principles

- Cryptography is centralized and never duplicated
- Protocol rules are explicit and enforced by code
- UI and transport layers are replaceable
