# NexOS Roadmap

## Phase 0 — Boot foundation
- [x] Rust workspace
- [x] NexKernel crate
- [x] x86_64 target
- [x] UEFI image generation
- [x] BIOS image generation
- [x] QEMU launch path
- [x] CI build

## Phase 1 — Kernel fundamentals
- [ ] CPU initialization and architecture abstraction
- [ ] GDT and TSS
- [ ] IDT and exception handling
- [ ] Physical memory manager
- [ ] Virtual memory manager
- [ ] Kernel heap
- [ ] Timer and clock source

## Phase 2 — Execution
- [ ] Scheduler
- [ ] Threads
- [ ] Processes
- [ ] Address-space isolation
- [ ] Syscall ABI
- [ ] IPC

## Phase 3 — Storage
- [ ] PCI discovery
- [ ] Block-device layer
- [ ] NVMe driver
- [ ] VFS
- [ ] Persistent filesystem
- [ ] Persistent userspace storage

## Phase 4 — Userspace
- [ ] Init
- [ ] Process launcher
- [ ] System libraries
- [ ] Shell
- [ ] Core utilities

## Phase 5 — Hardware
- [ ] Framebuffer console
- [ ] Keyboard
- [ ] Mouse
- [ ] USB
- [ ] Audio
- [ ] ACPI
- [ ] Power management

## Phase 6 — Networking
- [ ] NIC abstraction
- [ ] Ethernet
- [ ] IPv4 and IPv6
- [ ] UDP and TCP
- [ ] DNS
- [ ] Sockets

## Phase 7 — Desktop
- [ ] Compositor
- [ ] Window manager
- [ ] Desktop shell
- [ ] Settings
- [ ] File manager

## Phase 8 — Distribution
- [ ] Installer
- [ ] Signed release artifacts
- [ ] Reproducible ISO pipeline
- [ ] Hardware compatibility matrix
- [ ] Recovery environment
- [ ] Upgrade mechanism

No roadmap item is complete merely because an API or placeholder exists.
