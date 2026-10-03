# NexOS

**NexOS is an independent, real-world operating system being built from the ground up.**

NexOS uses its own kernel, **NexKernel**, with Rust as the primary implementation language and x86_64 as the first hardware architecture. The project is intended to progress from a bootable kernel to a complete installable operating system for supported laptops and PCs.

> **Status: Development / experimental.**
>
> NexOS is **not yet a Windows/Linux replacement or daily-driver operating system**. The repository contains real kernel, memory-management, process, terminal, filesystem, graphics, PCI/networking, and boot-image foundations, but hardware coverage and userspace are still incomplete.


## Goals

- Own kernel and kernel/user boundary
- Real multitasking and process isolation
- Real memory management
- Real storage and filesystem support
- Real hardware drivers
- Real networking
- Native userspace and shell
- Graphical desktop
- Installable x86_64 ISO
- Reproducible and signed release artifacts

## Architecture

    Firmware
      |
      v
    UEFI / BIOS
      |
      v
    Bootloader
      |
      v
    NexKernel
      |
      +-- memory
      +-- interrupts
      +-- scheduler
      +-- processes
      +-- syscalls
      +-- IPC
      +-- VFS
      +-- drivers
      +-- networking
      |
      v
    NexOS userspace

The bootloader is a loading component; it is not the NexOS kernel. NexKernel is developed independently in this repository.

## Current milestone

NexOS currently has real foundations for:

- x86_64 bootable NexKernel
- bootloader BootInfo and memory-map integration
- physical-frame allocation and page-table foundations
- process, syscall, and ELF-loader foundations
- interactive kernel terminal and keyboard input
- VFS/filesystem foundations
- framebuffer and graphics/window-manager foundations
- PCI hardware discovery
- network-device and VirtIO transport foundations
- Ethernet/ARP/IPv4/UDP/TCP/DHCP/DNS/socket foundations
- automated CI and development image generation

These are development components, not yet a complete general-purpose desktop OS.


## Build

Install QEMU on Debian or Ubuntu:

    sudo apt update
    sudo apt install qemu-system-x86

Then:

    cargo build --release

Run with UEFI:

    cargo run -- uefi

Run with legacy BIOS:

    cargo run -- bios

Build only the kernel:

    cargo build --package nexkernel --target x86_64-unknown-none

## Release readiness

The current v0.1.0 line is a **development milestone**, not a daily-driver release.

Before publishing a user-facing ISO release, NexOS must have:

1. Reliable kernel and userspace boot.
2. Persistent storage and filesystem support.
3. Installer and recovery environment.
4. Real RX/TX on documented supported NICs.
5. Documented supported Wi-Fi/Bluetooth hardware with drivers and firmware support.
6. Keyboard, mouse, display and graphics support on supported hardware.
7. Isolated user processes and working application execution.
8. End-to-end networking.
9. CI-produced bootable ISO artifacts.
10. QEMU and physical-hardware validation.

A GitHub Actions success alone does not mean NexOS is ready to replace Windows or Linux.


## Development

See docs/ARCHITECTURE.md, docs/BUILDING.md, docs/ROADMAP.md, SECURITY.md, and CONTRIBUTING.md.

## License

Apache-2.0

## Maintainer

Kritik Bhattarai

Project inquiries and security coordination: hunterkritik@gmail.com
