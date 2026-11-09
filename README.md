
# Gideon Mesh — Satellite Pass Terminal

Offline-first terminal module for **Gideon Mesh**.

YODPAY is the first product of Gideon Mesh.  
This repository is the satellite transport extension of the same architecture:

- work locally with no network
- queue offline state under hard limits
- burst-sync only when an uplink window exists
- stay silent when isolated

Philosophy: **Servers can be altered. Physics can’t.**

---

## What this is

A lightweight offline terminal that:

1. Estimates simplified satellite pass windows from time + local coordinates  
2. Accumulates offline records under tier/cap limits  
3. Enters hard-lock if bounds are breached  
4. Sends a burst only during a pass window  
5. Uses a C++ entropy filter to drop low-value noise before queueing

This is designed for constrained environments anywhere in the world.  
Coordinates are inputs, not hardwired to one country.

---

## Repository structure

```text
gideon-satellite-terminal/
├── Cargo.toml
├── src/
│   └── main.rs
├── filter.cpp
└── README.md
