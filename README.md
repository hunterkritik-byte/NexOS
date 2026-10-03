# NexOS

**NexOS is an experimental, from-scratch operating-system project written primarily in Rust.** NexKernel currently targets x86_64 and uses the Rust bootloader crate as its loading layer.

> **Status: early development.** NexOS is not a daily-driver OS. The repository contains boot, terminal, memory-management, filesystem, process, graphics, and networking foundations, but several are incomplete or not connected to working hardware drivers.

## What currently works (and what does not)

- **Boot:** BIOS and UEFI disk-image generation is provided by the image-builder. Test images in QEMU first.
- **Kernel terminal:** a basic VGA text terminal accepts PS/2 keyboard input. COM1 serial output is initialized; the development shell also polls serial input.
- **Shell:** commands include `help`, `clear`, `echo`, `uname`, `version`, `status`, `pwd`, `ls`, `cat`, `touch`, `mkdir`, `write`, `net`, `wifi`, `hotspot`, `bluetooth`, and `reboot`. `cat`, `touch`, `mkdir`, and `write` use the experimental in-memory VFS; their files disappear on reboot. `ls` currently shows the root namespace only. Network status commands report unavailable hardware support honestly; they do not enable those services.
- **Memory/processes:** boot memory-map access, frame allocation, page-table helpers, ELF inspection/loading, and syscall/process foundations exist. Do not assume full isolation or general-purpose multitasking.
- **Storage:** an in-memory VFS and experimental block/persistent-filesystem code exist. A complete, tested disk-backed filesystem and user-facing file commands are not yet available.
- **Networking:** PCI network-controller discovery and protocol-layer foundations exist. The current VirtIO network driver deliberately refuses to enable DMA until correct physical-address mapping and queue programming are implemented. No usable NIC is currently brought online by the boot path.
- **Wi-Fi hotspot:** not implemented. It needs a supported Wi-Fi chipset driver, firmware handling, 802.11 management, AP mode, authentication, and DHCP/network integration.
- **Bluetooth:** not implemented. It needs a supported controller transport/driver and HCI plus higher-level Bluetooth protocols.
- **Graphics:** framebuffer and windowing foundations exist; this is not yet a complete desktop environment.

See [docs/ROADMAP.md](docs/ROADMAP.md) for tracked status, [docs/SHELL.md](docs/SHELL.md) for shell limits, [docs/NETWORKING.md](docs/NETWORKING.md) for network plans, and [docs/OS-READINESS.md](docs/OS-READINESS.md) for the staged path toward a general-purpose OS.

## Architecture

```text
Firmware (BIOS / UEFI)
        |
     Bootloader
        |
     NexKernel
        +-- x86_64 architecture code
        +-- memory and page tables
        +-- interrupts and syscalls
        +-- process/ELF foundations
        +-- terminal and shell
        +-- VFS and storage foundations
        +-- PCI and network foundations
        |
     NexOS userspace (early)
```

The bootloader is a loading component, not the NexOS kernel. NexKernel is developed independently in this repository.

## Build

Requirements: Rust nightly (pinned by `rust-toolchain.toml`) and an x86_64 host. Install QEMU on Debian/Ubuntu for virtual-machine testing:

```sh
sudo apt update
sudo apt install qemu-system-x86 ovmf xorriso
```

Build the kernel and userspace shell:

```sh
cargo build --release --package nexkernel --target x86_64-unknown-none
cargo build --release --package nexshell --target x86_64-unknown-none
```

Create BIOS and UEFI disk images:

```sh
mkdir -p dist
cargo run --release --package nexos-image-builder -- \
  target/x86_64-unknown-none/release/nexkernel \
  target/x86_64-unknown-none/release/nexshell \
  dist/NexOS-x86_64-bios.img \
  dist/NexOS-x86_64-uefi.img
```

Run the BIOS image in QEMU:

```sh
qemu-system-x86_64 -drive format=raw,file=dist/NexOS-x86_64-bios.img -serial stdio
```

For UEFI, use OVMF firmware and a writable OVMF variables image appropriate for your distribution. Exact firmware paths vary by host. See [docs/BUILDING.md](docs/BUILDING.md).

The CI-generated `NexOS-x86_64.iso` currently packages the BIOS and UEFI disk images as files; it is **not a directly bootable hybrid ISO**. Boot the `.img` files with the corresponding firmware.

## Release policy

A Git tag or successful CI run does not by itself mean NexOS is ready for daily use. Before calling a release hardware-ready, the project needs reproducible builds, verified boot tests, a documented hardware compatibility matrix, working storage and user applications, and end-to-end tests for each advertised device.

Do not install development images on a production disk. Prefer QEMU or disposable hardware.

## Development

- [Architecture](docs/ARCHITECTURE.md)
- [Build instructions](docs/BUILDING.md)
- [Roadmap](docs/ROADMAP.md)
- [Networking, Wi-Fi, hotspot, and Bluetooth](docs/NETWORKING.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## License

Apache-2.0

## Maintainer

Kritik Bhattarai
