# NexOS Architecture

NexOS is being developed as an independent operating system. The kernel is NexKernel; it is not based on the Linux or BSD kernels.

## Current execution path

UEFI or BIOS -> bootloader -> NexKernel -> early serial console

The Rust bootloader crate is used as the loading layer. NexKernel is the operating-system kernel being developed in this repository.

## Kernel boundaries

Planned kernel subsystems:

- arch: CPU and architecture-specific code
- mm: physical and virtual memory
- interrupts: exceptions and interrupt routing
- sched: processes, threads, and scheduling
- syscall: user/kernel interface
- ipc: inter-process communication
- fs: VFS and filesystems
- drivers: hardware drivers
- net: networking
- security: isolation and hardening

A subsystem is considered implemented only after it performs real work on the target machine.
