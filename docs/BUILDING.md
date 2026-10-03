# Building NexOS

## Requirements

- x86_64 development host
- Rust nightly pinned by `rust-toolchain.toml`
- QEMU for virtual-machine testing
- OVMF and xorriso for UEFI testing and ISO packaging

On Debian or Ubuntu:

```sh
sudo apt update
sudo apt install qemu-system-x86 ovmf xorriso
```

## Build the kernel and userspace shell

```sh
cargo build --release --package nexkernel --target x86_64-unknown-none
cargo build --release --package nexshell --target x86_64-unknown-none
```

## Create BIOS and UEFI disk images

The image builder requires four positional arguments: kernel ELF, NexShell ELF, BIOS output path, and UEFI output path.

```sh
mkdir -p dist
cargo run --release --package nexos-image-builder -- \
  target/x86_64-unknown-none/release/nexkernel \
  target/x86_64-unknown-none/release/nexshell \
  dist/NexOS-x86_64-bios.img \
  dist/NexOS-x86_64-uefi.img
```

## Run in QEMU

BIOS:

```sh
qemu-system-x86_64 \
  -drive format=raw,file=dist/NexOS-x86_64-bios.img \
  -serial stdio
```

UEFI requires OVMF code and a writable variables image. Firmware file locations vary by distribution. Use separate code and variables files; do not modify the system's shared OVMF variables file.

## CI and release artifacts

GitHub Actions builds the kernel and NexShell, creates BIOS and UEFI disk images, and packages the images in an ISO for convenient distribution. The current ISO is a data ISO containing the disk images; it is not a directly bootable hybrid ISO. Boot the disk image itself in QEMU.

Do not install early development images onto a production disk. Test in QEMU and on disposable hardware first.
