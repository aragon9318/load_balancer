# ADR-0001: Use Operating System TCP Stack

## Status
Accepted

## Context
The project requires reliable TCP-based transport as the foundation of its
networking layer. One possible approach is to implement a custom TCP stack in
kernel space to gain fine-grained control over packet processing and Layer-3
interactions. However, kernel-level networking introduces significant
complexity, operational risk, and development overhead.

At the current stage of the project, stability, portability, and development
velocity are higher priorities than low-level packet control.

## Decision
The project will use the **operating system–provided TCP stack** via the Rust
standard library instead of implementing a custom TCP stack in kernel space.

TCP connection handling, congestion control, retransmission, IP processing, and
checksum computation are delegated to the operating system. The project’s
networking layer operates entirely in user space and remains agnostic of
kernel-level transport details.

## Rationale
- The OS TCP stack is a **mature, well-tested, and widely deployed
  implementation**, reducing the risk of transport-layer defects.
- Kernel-space networking errors can result in **system instability, kernel
  panics, or unrecoverable faults**, which are disproportionate risks at this
  stage of the project.
- Leveraging the standard TCP stack allows development to focus on
  **architecture, data-plane abstractions, and control-plane design** rather
  than low-level transport correctness.
- This approach aligns with common industry practices for early-stage and
  production systems that do not require custom transport semantics.


## Consequences
- The system does not have direct visibility into **Layer-3 (IP) or Layer-4
  header fields** prior to TCP session establishment. As a result, the data
  plane cannot make early admission or filtering decisions based on source IP
  addresses, TCP flags, or packet-level metadata.
- Because connection handling relies on the operating system’s TCP stack,
  **TCP state is allocated by the kernel before user-space logic is invoked**.
  This limits the system’s ability to prevent certain classes of attacks at
  early stages of the connection lifecycle.
- The system may be exposed to transport-layer and network-layer attacks such
  as **SYN floods, connection exhaustion, denial-of-service (DoS), and
  distributed denial-of-service (DDoS) attacks**, where large numbers of
  connection attempts are initiated before application-level rejection can
  occur.
- While application-level mitigations (e.g., connection limits, rate limiting,
  authentication gating, and early connection teardown) can be implemented in
  user space, they occur **after the TCP three-way handshake has completed**.
  Consequently, these mechanisms cannot fully prevent resource allocation at
  the kernel level.
- Mitigations that operate at **Layer-3 or early Layer-4 stages**—such as
  packet filtering, SYN cookies, or ingress traffic shaping—must be provided
  by the operating system or external infrastructure (e.g., firewall rules,
  load balancers, or network appliances), as they are outside the scope of the
  user-space TCP implementation.

## Future Considerations
If future requirements demand lower-level packet visibility, custom congestion
control, or specialized transport semantics, this decision may be revisited.
Possible alternatives include:
- User-space networking frameworks
- eBPF-based extensions
- Custom kernel modules
- Alternative transport protocols

Any such change should be justified by clearly defined performance or
functional requirements.
