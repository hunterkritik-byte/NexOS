# NexOS

**NexOS Linux is a practical Linux distribution project built on the Debian GNU/Linux ecosystem, with NexOS-owned tooling, defaults, packaging, security policy, installer experience, and release engineering.**

> **Current direction:** NexOS is moving from the original from-scratch Rust-kernel experiment to a real Linux-based distribution. The Rust NexKernel remains a separate research track; it is not the kernel used by NexOS Linux.

## What NexOS is

NexOS Linux aims to provide an Ubuntu-like end-user experience while keeping a clean Debian foundation and owning the distribution layer.

Current foundation:

- Debian GNU/Linux 13 "trixie" stable
- Linux kernel + systemd
- amd64 first
- Xfce desktop for the initial desktop edition
- NetworkManager
- Debian Live / live-build image pipeline
- Calamares installer foundation
- NexOS branding and system metadata
- Reproducible build/checksum pipeline
- GitHub Actions ISO validation

Debian 13.7 is the current Debian stable point release as this work begins.

## Editions planned

| Edition | Goal |
| --- | --- |
| **Core** | Minimal command-line NexOS for servers and recovery |
| **Desktop** | Friendly Xfce desktop for everyday use |
| **Developer** | Desktop + compilers, debugging, containers and NexOS tooling |
| **Future** | Custom desktop shell, additional architectures and NexOS kernel configuration |

## Build the Linux distribution

On Debian/Ubuntu:

```sh
sudo apt update
sudo apt install live-build debootstrap qemu-system-x86 xorriso squashfs-tools
sudo bash ./scripts/build-linux.sh
```

The resulting image is:

```
dist/NexOS-Linux-amd64.iso
dist/NexOS-Linux-amd64.iso.sha256
```

For a smaller command-line image:

```sh
sudo bash ./scripts/build-linux.sh --no-desktop
```

The build uses Debian Live's live-build tooling to automate customized Debian live-system images.

## Test in QEMU

```sh
./scripts/smoke-test.sh dist/NexOS-Linux-amd64.iso
```

Do not install development images onto a production disk.

## Architecture

```
Firmware
   |
GRUB / Debian Live
   |
Linux kernel
   |
systemd
   |
Debian userspace
   |
NexOS system configuration + packages
   |
NexOS desktop / tools
   |
Applications
```

The original Rust track remains available for OS research.

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md) and [docs/LINUX-DISTRO.md](docs/LINUX-DISTRO.md).

The immediate goal is a bootable, installable, QEMU-tested NexOS Linux image before adding deeper custom platform features. The CI pipeline builds the downloadable amd64 ISO and validates its boot structure before publishing the artifact.

## Development

- [Linux distribution architecture](docs/LINUX-DISTRO.md)
- [Roadmap](docs/ROADMAP.md)
- [Original OS architecture](docs/ARCHITECTURE.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## License

Apache-2.0

## Maintainer

Kritik Bhattarai
