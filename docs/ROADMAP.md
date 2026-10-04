# NexOS Roadmap

This roadmap distinguishes source-code foundations from features that have been integrated and tested on a running machine. A module or API alone does not count as a working feature.

## Phase 0 — Build and boot foundation
- [x] Rust workspace and pinned nightly toolchain
- [x] NexKernel x86_64 crate
- [x] BIOS and UEFI disk-image generation code
- [ ] Keep formatting, workspace checks, and kernel builds green in CI
- [ ] Verify serial boot output and successful handoff in QEMU for both firmware paths
- [ ] Make release artifacts reproducible and validate image contents

## Phase 1 — Kernel fundamentals
- [x] GDT/TSS and IDT setup foundations
- [x] Bootloader memory-map integration
- [x] Usable physical-frame allocator foundation
- [x] Active page-table mapper foundation
- [ ] Kernel heap allocator
- [ ] Robust exception handlers with useful diagnostics
- [ ] Timer/clock source and timer interrupts
- [ ] SMP and per-CPU initialization

## Phase 2 — Execution and isolation
- [x] Process metadata and fixed-capacity scheduler prototype
- [x] ELF64 header inspection and user-process mapping foundations
- [x] Syscall entry and user launch foundations
- [ ] Demonstrate safe ring-3 execution and return to the kernel
- [ ] Preemptive scheduling and context switching
- [ ] Enforce per-process address-space isolation and permissions
- [ ] Process exit, wait, signals/events, and IPC
- [ ] Regression tests for syscall boundary validation

## Phase 3 — Terminal and userspace
- [x] VGA text output and scrolling
- [x] Basic PS/2 keyboard input
- [x] Basic kernel shell command loop
- [x] COM1 serial output and polling input path
- [ ] Add robust keyboard layouts, shift/caps handling, and reliable line editing
- [ ] Connect userspace input/read syscalls to a real terminal
- [ ] Make NexShell an interactive shell instead of a print-and-yield stub
- [x] Add basic VFS-backed `cat`, `touch`, `mkdir`, and `write` shell commands (in-memory only)
- [ ] Add directory-aware `ls`, functional `cd`, and a broader set of file operations
- [ ] Add init/service lifecycle and core utilities

## Phase 4 — Storage
- [x] VFS API and in-memory filesystem prototype
- [x] Block-device abstraction and ATA PIO prototype (not yet proven on hardware)
- [x] Fixed-slot persistent filesystem with multi-sector metadata and bounded file reads/writes (not yet integrated into boot path or hardware-tested)
- [ ] Add a real block-device driver and safe device discovery
- [ ] Validate on-disk metadata, allocation, bounds, and crash consistency
- [ ] Implement a complete, tested filesystem and persistent userspace storage
- [ ] Add a storage test suite and recovery behavior

## Phase 5 — Wired networking
- [x] PCI configuration-space discovery foundation
- [x] Network-device abstraction and protocol-layer modules
- [x] Ethernet/ARP/IPv4/UDP/TCP/DHCP/DNS/socket code foundations
- [ ] Implement at least one supported NIC driver with real RX/TX
- [ ] Correct VirtIO queue setup, physical DMA addresses, and memory barriers
- [ ] Add packet-buffer ownership, timeouts, and malformed-packet tests
- [ ] Integrate DHCP, DNS, and sockets with an active NIC
- [ ] Validate end-to-end connectivity in QEMU

## Phase 6 — Wi-Fi, hotspot, and Bluetooth
- [ ] Select and document specific supported Wi-Fi chipsets
- [ ] Implement PCI/USB transport and firmware loading for selected Wi-Fi hardware
- [ ] Implement 802.11 scanning, association, authentication, and key management
- [ ] Add a Wi-Fi control service and network configuration interface
- [ ] Implement AP mode, authentication, and DHCP/NAT before advertising hotspot support
- [ ] Select supported Bluetooth controllers and implement HCI transport
- [ ] Implement required Bluetooth protocols and device pairing
- [ ] Test on physical hardware and document limitations

## Phase 7 — Graphics and input
- [x] Framebuffer/windowing foundations
- [x] Fixed-capacity window manager with stable IDs, z-order, focus, hit testing, move/resize, and close operations (host-side regression tests added; compositor not integrated)
- [ ] Hardware-independent terminal rendering
- [ ] USB HID keyboard/mouse support
- [x] Host-testable desktop pointer routing: primary-click focus and title-bar dragging (requires a driver to feed mouse events; not wired into boot)
- [x] Software compositor foundation that paints desktop background, window borders, and active/inactive title bars (host-side tests added; framebuffer and input integration still pending)
- [x] Allocation-free bitmap graphics text console with clipped drawing and configurable colors (host-side tests added; boot shell integration still pending)
- [ ] Window manager integration with live framebuffer and desktop shell
- [ ] Settings and file manager

## Phase 8 — Distribution and release
- [ ] A genuinely bootable ISO or clearly documented disk-image distribution
- [ ] Release CI that fails on unsuccessful boot tests (not just QEMU timeout)
- [ ] SHA-256 checksums and release notes
- [ ] Hardware compatibility matrix and firmware/licensing notes
- [ ] Installer, recovery environment, and upgrade strategy
- [ ] End-to-end tests on QEMU and supported physical hardware

## v0.2.0 release gate

Do not describe Wi-Fi, hotspot, Bluetooth, persistent storage, multitasking, or desktop support as working until the corresponding end-to-end tests pass. The immediate release blockers are a green formatting/build pipeline, reliable BIOS/UEFI boot validation, and an interactive terminal/userspace path.
