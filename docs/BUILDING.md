# Building NexOS

## Requirements

- x86_64 development host
- Rust nightly pinned by rust-toolchain.toml
- QEMU for virtual-machine testing

On Debian or Ubuntu:

    sudo apt update
    sudo apt install qemu-system-x86

## Build

    cargo build --release

## Run in QEMU

UEFI:

    cargo run -- uefi

Legacy BIOS:

    cargo run -- bios

## Kernel-only build

    cargo build --package nexkernel --target x86_64-unknown-none

Do not install early development images onto a production disk. Test in QEMU and on disposable hardware first.
