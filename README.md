# Load Balancer (Rust)

## Overview
This project demonstrates the capabilities of **Rust** for handling TCP connections efficiently and securely.  
It is intended as a **learning and portfolio project**, and **is not production-ready**.  

The project will **evolve over time**, adding:

- TLS/SSL support  
- HTTP/HTTPS handling  
- Advanced routing with virtual IPs  
- Security features (IP allow/block, rate limiting)  
- Metrics and observability  

Currently, provides a **scaffold and basic TCP listener** for FreeBSD.  
All architectural considerations, security evaluation, and kernel vs user-space research are documented in the `docs/` folder.

---

## Goals

- Demonstrate **Rust’s performance and safety** in connection handling  
- Provide **modular architecture** for evolving load balancer features  
- Keep **secure defaults** for future extensions  
- Maintain **clear documentation** for design decisions, threat models, and metrics  

---


## Architecture Overview

This project follows a **Control Plane / Platform Plane / Data Plane** architecture,
inspired by networking principles described in **RFC 7426**.

- **Control Plane**: decision-making, configuration, policies
- **Platform Plane**: safe state distribution and atomic updates
- **Data Plane**: high-performance connection and traffic handling

                         ┌───────────────────────────────┐
                         │           Control Plane       │
                         │───────────────────────────────│
                         │  Config Loading & Validation  │
                         │  Routing Rules & Policies     │
                         │  Security Decisions           │
                         │  Metrics Aggregation          │
                         │                               |
                         └───────────────┬───────────────┘
                                         │
                                         │
                         ┌───────────────▼───────────────┐
                         │          Platform Plane       │
                         │───────────────────────────────│
                         │  Shared Runtime State         │
                         │  Atomic Swaps                 │
                         │  Hot Reload Mechanisms        │
                         │  Safe State Distribution      │
                         │                               │
                         │  (Arc, Atomics, ArcSwap)      │
                         └───────────────┬───────────────┘
                                         │
                                         │
 ┌───────────────────────────────────────▼──────────────────────────────────┐
 │                                Data Plane                                 │
 │───────────────────────────────────────────────────────────────────────────│
 │  TCP Listener & Connection Handling                                       │
 │  HTTP Parsing & Validation                                                │
 │  TLS Termination (future)                                                 │
 │  Traffic Forwarding & I/O                                                 │
 └───────────────────────────────────────┬──────────────────────────────────┘
                                         │
                                         ▼
                           ┌──────────────────────────┐
                           │         Backends         │
                           │  Application Servers     │
                           └──────────────────────────┘



This separation allows the data plane to operate with minimal overhead while the
control plane can evolve dynamically without restarting the application.

For detailed design, see `docs/architecture.md`.




## Disclaimer

This project is intended for **educational and demonstration purposes only**.
It is **not designed for production use** and does not replace enterprise-grade
load balancers.

---

## License

MIT License
