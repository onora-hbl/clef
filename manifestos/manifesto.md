# Manifesto — Clef

## Why This Project Exists

Modern communication platforms centralize power, visibility, and control.
They decide who can speak, what can be seen, and what can be remembered.

This project exists to prove that **another model is possible**.

A model where:

- communities can self-host without fragmentation
- privacy is structural, not a feature toggle
- usability does not collapse under cryptographic complexity

The goal is not to build a niche tool for experts,
but a system that **feels obvious to users and hostile to surveillance**.

This manifesto defines the *intent*, not a frozen design.
Everything here can evolve — but the pillars do not.

---

## Our Vision

Clef is a decentralized, encrypted, real-time communication ecosystem
designed to feel as simple as Discord while being fundamentally different underneath.

Users should not need to understand instances or cryptography
to enjoy the system.

Those concepts exist **to protect them**, not to burden them.

---

## Core Pillars

### 1. Decentralization Without Fragmentation

Anyone can run an instance.

An instance:

- hosts community servers
- stores encrypted data
- enforces **structural rules** (access control, rate limits)

An instance administrator:

- decides who can create servers locally
- can moderate structurally (ban, isolate, rate-limit)
- **cannot read or interpret user content**

Decentralization must increase resilience and autonomy,
not split the network into incompatible silos.

---

### 2. Privacy by Construction

Privacy is not a policy.
It is a *consequence of architecture*.

- All user-generated content is encrypted end-to-end
- Encryption happens on clients
- Decryption happens on clients
- Instances never possess the material required to read content

The instance is **blind to meaning**, but **aware of structure**.

This distinction is essential.

---

### 3. Simplicity for Humans

From a user perspective:

- one identity
- one contact list
- one list of servers
- one continuous history
- seamless multi-device usage

Users do not manage instances.
They do not manage keys.
They do not resync manually.

The system absorbs complexity so users do not have to.

---

## What the Instance Is — and Is Not

### The Instance Is:

- a storage and relay layer
- an authority for *structural rules*
- a coordinator for permissions

### The Instance Is Not:

- a trusted party
- a content moderator
- a cryptographic authority
- a data owner

**The instance enforces rules, not meaning.**

It decides *whether* something may be sent or retrieved,
never *what* it contains.

---

## Structural vs Semantic Control

This project explicitly separates:

- **Structural logic** (allowed / not allowed)
- **Semantic content** (what messages say)

Examples:

- An instance may refuse to deliver channel data to a user without access
- An instance may ban an identity from participating

But it cannot:

- read messages
- analyze media
- detect keywords
- inspect conversations

This is a deliberate limitation, not a weakness.

---

## Client Philosophy

The client is an interface — not an authority.

- Clients do not define truth
- Clients do not override protocol rules
- Clients do not hold special privileges

All logic that affects security or access exists at the **protocol level**,
verifiable and reproducible by any implementation.

---

## Open by Default

The entire ecosystem is open:

- protocols
- reference implementations
- governance
- architectural decisions

Power comes from transparency, not control.

---

## Our Commitment

We commit to:

- never weakening privacy for convenience
- never adding backdoors, even “optional” ones
- never centralizing identity or trust
- never tying users to a single instance or client

If a trade-off must be made, **privacy and autonomy win**.