# Clef

**Clef** (/kle/) is an experimental, decentralized communication platform
designed as a privacy-first alternative to Discord.

Clef provides familiar real-time features — servers, channels, roles, private messages, and multi-device sync —
while rejecting centralization, surveillance, and opaque authority.

The goal is simple:
**make secure, decentralized communication feel obvious.**

---

## Core Pillars

Clef is built on three non-negotiable principles:

- **Decentralization that works**  
  Anyone can run an instance. No central authority. No privileged servers.

- **Privacy by construction**  
  All user content is end-to-end encrypted. Instances cannot read messages, media, or profiles.

- **Simplicity for users**  
  One identity, seamless multi-device usage, instant history access when authorized — without exposing infrastructure complexity.

---

## Why “Clef”?

A _clef_ is a key — not the music itself, but what makes it readable.

In Clef:

- **structure is visible**
- **meaning is private**
- **access is granted by possession, not trust**

The name reflects the project’s core idea:
**keys unlock communication, but infrastructure never owns it.**

---

## Project Philosophy

Clef is guided by two complementary documents:

- 📜 **[Manifesto](./manifestos/manifesto.md)**  
  The vision, values, and goals of the project.

- 🛠 **[Technical Manifesto](./manifestos/technical-manifesto.md)**  
  The architectural principles that guarantee decentralization, privacy, and usability.

These documents define what Clef _must_ be — and what it must never become.

---

## Discussions

Don’t hesitate to jump into the **[Discussions](https://github.com/onora-hbl/clef/discussions/categories/general-discussions)** tab to talk about the project.

Any topic is welcome:

- questions about the protocol or cryptographic model
- design ideas or alternative approaches
- doubts, critiques, or “what if?” scenarios
- early thoughts that aren’t ready to become issues or PRs yet

This is a **benevolent, open space** meant for exploration and collective thinking.  
There are no bad questions — if something is unclear, it’s worth discussing.

---

## Contributing

Clef is fully open-source and community-driven.

Anyone is welcome to contribute — code, design, documentation, or discussion —
as long as contributions respect the project’s architectural principles.

Before contributing, please read:

➡️ **[CONTRIBUTING.md](./CONTRIBUTING.md)**

All technical and architectural decisions are discussed openly.

---

# Project Documentation

All technical and design documentation for this project is centralized in the `docs/` folder.

You can start exploring the documentation here:

- [Go to Docs](./docs/README.md)

This will give you access to:

- API specifications
- Architecture overviews
- Protocol details

---

## Codebase

The Clef codebase is primarily written in **Rust** and organized as a **multi-crate workspace**.

All Rust code lives under the `crates/` directory, where each crate has a
clear, single responsibility (cryptography, protocol rules, client logic, server logic, UI).

This structure is designed to:

- isolate security-critical code
- make protocol rules explicit and testable
- keep UI and transport layers decoupled from cryptography

For a detailed overview of the code architecture and the role of each crate,
see the dedicated documentation:

**[Code Architecture](crates/README.md)**

---

## License

Clef is licensed under the **GNU Affero General Public License v3 (AGPLv3)**.

This means:

- the code is free and open-source
- all modifications and forks must remain open-source
- using Clef as a network service requires publishing source code changes

See **[LICENSE](./LICENSE)** for full details.

---

## Status

Clef is in early exploration and design phase.
Expect sharp edges, evolving ideas, and open questions.

If you’re interested in privacy, decentralization, and building something better —
you’re in the right place.
