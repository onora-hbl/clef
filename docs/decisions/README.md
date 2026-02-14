# Technical Decisions

This folder documents **concrete, binding technical decisions** made for the Clef project.

These decisions define:
- which technologies are used
- which cryptographic primitives are selected
- which data models are adopted
- which approaches are explicitly rejected

They are **practical choices**, not theoretical discussions.

---

## What This Category Covers

This folder records decisions such as:

- cryptographic algorithms and key sizes
- protocol adaptations and simplifications
- data storage models (databases, indexing strategies)
- serialization formats
- networking and transport choices
- client and server implementation constraints

These decisions directly impact:
- security guarantees
- performance characteristics
- implementation complexity
- long-term maintainability

---

## What This Category Does *Not* Cover

- Project vision or values → see the Manifesto
- High-level architectural principles → see the Technical Manifesto
- Protocol definitions and flows → see the Protocol Specification
- API design and client interfaces → see the API Reference
- Temporary implementation experiments

This folder answers the question:

> **“What did we choose to use, concretely — and why?”**

---

## Decision Philosophy

Each documented decision is:

- **explicit** — no hidden assumptions
- **dated** — context matters
- **justified** — trade-offs are stated
- **revisable** — changes require a new decision record

Reversing a decision is allowed,
but only by documenting a new one.

---

## Document Structure

Each decision document should include:

- **Decision title**
- **Date**
- **Context**
- **Decision**
- **Consequences**

The goal is clarity, not exhaustiveness.

---

## Example Decisions

Examples of appropriate decision topics:

- Choice of symmetric encryption algorithm
- Database model for encrypted blobs
- Client-side state persistence strategy
- Server-side language and framework constraints

---

## Why This Matters

In a security-sensitive system, 
**implicit technical choices are liabilities**.

Documenting concrete decisions:
- prevents accidental weakening of guarantees
- accelerates onboarding of contributors
- creates a shared technical baseline
- makes architectural evolution intentional

This folder is the project’s **technical source of truth**.
