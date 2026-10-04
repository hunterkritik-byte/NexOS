# NexOS Linux distribution

NexOS is transitioning from the original from-scratch Rust-kernel experiment into a practical Linux distribution.

## Base

- Debian GNU/Linux 13 "trixie" stable
- amd64 first
- systemd
- Xfce desktop for the first desktop edition
- Debian APT repositories plus NexOS-owned packages/repositories
- live-build for reproducible live/install media

Debian 13.7 is the current stable base at the time this design was started. NexOS owns the distribution layer: branding, defaults, packages, configuration, installer experience, update policy, security defaults, tooling, and release artifacts.

The old NexKernel/Rust OS code remains in this repository as an experimental research track. It is not the kernel of NexOS Linux.

## Build

On Debian/Ubuntu:

    sudo apt update
    sudo apt install live-build debootstrap qemu-system-x86 xorriso squashfs-tools
    sudo ./scripts/build-linux.sh

Output:

    dist/NexOS-Linux-amd64.iso

For a lightweight build:

    sudo ./scripts/build-linux.sh --no-desktop

For a VM smoke test:

    ./scripts/smoke-test.sh dist/NexOS-Linux-amd64.iso
