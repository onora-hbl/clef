# Technical Manifesto — Clef

## Preamble

This document explains *how* the system upholds the manifesto pillars
without relying on trust in instances, administrators, or clients.

It intentionally avoids algorithm-level detail
and instead focuses on responsibilities, flows, and guarantees.

---

## Core Technical Principles

### 1. Zero Trust Architecture

No infrastructure component is trusted by default:

- instances
- administrators
- storage layers
- clients
- network

Security emerges from cryptographic proofs and protocol constraints,
not from assumed honesty.

---

### 2. The Instance Is Blind to Content, Not to Rules

This is a critical clarification.

The instance **cannot**:

- read messages or media
- decrypt metadata
- infer conversation meaning

But the instance **must**:

- know *who is allowed* to access a channel
- enforce joins, leaves, bans, and permissions
- decide whether encrypted data may be delivered

The instance enforces *access*, not *understanding*.

---

## Identity and Devices

### User Identity

Each user has a global cryptographic identity,
independent of any instance.

This identity:

- is used across all servers and instances
- cannot be impersonated by an instance
- anchors trust between users

---

### Devices

- A user may have multiple devices
- Each device has its own cryptographic identity
- Devices are explicitly linked to a user identity

Adding or removing a device is a **cryptographically authorized action**
performed by the user from any trusted device.

Instances store device state but cannot forge or alter it.

---

## Servers and Channels

### Servers

A server is a coordination domain hosted by an instance.

The instance manages:

- membership lists
- roles
- permission graphs
- structural moderation

But does not control cryptographic access.

---

### Channels

A channel is defined by:

- a structural access rule set (managed by the instance)
- an encrypted history (opaque to the instance)

To access a channel’s content,
a client must satisfy **both**:

1. instance-level permission checks  
2. possession of the required cryptographic material  

Failing either results in no data delivery.

---

## Message Flow (Conceptual)

1. A client encrypts a message locally
2. The encrypted message is sent to the instance
3. The instance verifies:
   - sender identity
   - sender permission for the channel
4. If authorized:
   - the encrypted message is stored and relayed
5. Recipients retrieve encrypted data
6. Decryption happens locally on clients

At no point can the instance read or reinterpret content.

---

## History Access

History is:

- stored encrypted
- indexed structurally (timestamps, channel IDs)
- inaccessible without proper authorization

When a user gains access to a channel:

- the instance allows retrieval of the encrypted history
- the client decrypts locally if it has the required material

If access is revoked:

- the instance stops delivering new data
- cryptographic rotation prevents future access

---

## Multi-Device Synchronization

When a user:

- joins a server on device A
- or gains access to a channel

The change is:

- recorded structurally by the instance
- cryptographically propagated to the user identity

Other devices (B, C, …):

- detect the updated state
- retrieve required encrypted material
- gain access transparently

No device needs to be online at the same time.
No device acts as a relay for another.

---

## Private Messages

Private messages:

- follow the same structural / cryptographic split
- work across instances
- do not require trust between instance administrators

Instances only know *that* two identities exchange encrypted data,
not what is exchanged.

--

## Security Boundaries

This system makes a clear and intentional distinction between
**unavoidable structural visibility** and **forbidden semantic access**.

An instance administrator may observe:

- traffic volume
- channel activity patterns (creation, activity timing)
- membership graphs (who has access to what)

This information exists because the instance is responsible for:

- enforcing access rules
- routing and storing encrypted data
- applying moderation at a structural level
- protecting itself against abuse (spam, DoS, resource exhaustion)

Hiding all structural metadata would require either:
- constant client-to-client availability, or
- a trusted central authority, or
- heavy anonymity networks that would severely degrade usability

All of these options contradict the project’s core goals.

---

### What the Instance Can *Never* Access

The instance may **not**, under any circumstances, observe:

- message content
- media content
- user profiles or status messages
- channel topics, names, or descriptions
- semantic metadata (keywords, reactions, meaning, intent)

This is enforced by architecture, not policy.

The instance does not possess the cryptographic material required to
interpret or reconstruct user data, even if fully compromised.

---

### Why This Trade-off Exists

Some information must remain visible for the system to function:

- messages must be routed
- permissions must be enforced
- storage must be allocated
- abuse must be mitigated

Attempting to hide *all* metadata would:

- make moderation impossible
- require trusted intermediaries
- break multi-device synchronization
- significantly increase latency and complexity

Instead of pretending this leakage does not exist,
the project chooses to:

- **minimize it**
- **document it**
- **design around it**
- **prevent it from becoming power**

---

### Boundary as a Safety Mechanism

The key security guarantee is not absolute invisibility,
but **containment of power**.

Knowing *that* a channel exists is not the same as knowing *what* is said.
Knowing *who* participates is not the same as controlling their speech.

By strictly separating structure from meaning:

- administrators can operate infrastructure
- communities can self-govern
- users retain privacy and autonomy

---

### Explicit Non-Goals

This project does **not** claim to provide:

- perfect anonymity
- deniability against global network observers
- protection against all traffic analysis

Those are separate, valuable problems —
but solving them here would compromise usability, decentralization,
or both.

---

## Evolution

- Cryptographic mechanisms can evolve
- Protocols are versioned
- Backward compatibility is explicit
- No upgrade may weaken privacy guarantees

---

## Final Summary

- Instances enforce rules, not meaning
- Clients encrypt, instances coordinate
- Access is structural, secrecy is cryptographic
- Users control identity, devices, and data

This is not about hiding information.
It is about **removing the power to see it**.
