# NexOS

<p align="center">
  <img src="https://raw.githubusercontent.com/hunterkritik-byte/NexOS/main/docs/assets/nexos-logo.png" alt="NexOS" width="180">
</p>

<p align="center">
  <strong>A practical Debian-based Linux desktop distribution built for everyday use.</strong>
</p>

<p align="center">
  <a href="https://github.com/hunterkritik-byte/NexOS/actions/runs/37272489178/artifacts/11328792769"><img src="https://img.shields.io/github/v/release/hunterkritik-byte/NexOS?display_name=tag&sort=semver" alt="Latest release"></a>
  <a href="https://github.com/hunterkritik-byte/NexOS/actions"><img src="https://img.shields.io/github/actions/workflow/status/hunterkritik-byte/NexOS/iso.yml?label=ISO%20build" alt="ISO build"></a>
  <a href="https://github.com/hunterkritik-byte/NexOS/blob/main/LICENSE"><img src="https://img.shields.io/github/license/hunterkritik-byte/NexOS" alt="License"></a>
</p>

## 💖 Sponsorship & Collaboration

**Sponsorship, hardware support, development partnerships, and collaboration:** **hunterkritik@gmail.com**

If you want to support NexOS development, sponsor hardware/testing, or collaborate on the project, please get in touch by email.

---

## ⬇️ Download NexOS

### NexOS v0.2.0 — First Desktop ISO

**Field test: ✅ Passed**

NexOS v0.2.0 is the first full NexOS Desktop ISO, targeting **x86_64/amd64** hardware.

<p align="center">
  <a href="https://github.com/hunterkritik-byte/NexOS/actions/runs/37272489178/artifacts/11328792769">
    <img src="https://img.shields.io/badge/%E2%AC%87%20Download-NexOS%20v0.2.0-blue?style=for-the-badge" alt="Download NexOS v0.2.0">
  </a>
</p>

**ISO size:** approximately **2.54 GB**

**Download:** [⬇️ Download NexOS Desktop x64 (2.54 GB)](https://github.com/hunterkritik-byte/NexOS/actions/runs/37272489178/artifacts/11328792769)  

[View NexOS Releases](https://github.com/hunterkritik-byte/NexOS/releases)

> The v0.1.0 release is the earlier 4.5 MB kernel/development image. **v0.2.0 is the first full desktop distribution ISO.**

### Verify the download

Download the `SHA256SUMS` file from the release and run:

```sh
sha256sum NexOS-x86_64-v0.2.0.iso
```

Compare the result with the checksum published in the release.

---

## 🖥️ What is NexOS?

**NexOS Linux** is a practical Linux distribution built on the Debian GNU/Linux ecosystem, with NexOS-owned tooling, defaults, packaging, security policy, installer experience, branding, and release engineering.

NexOS aims to provide a familiar, friendly desktop experience while retaining the stability and package ecosystem of Debian.

The original from-scratch Rust kernel experiment remains a separate research track and is **not** the kernel used by NexOS Linux.

## ✨ Features

- 🐧 Debian GNU/Linux foundation
- 🖥️ Xfce desktop environment
- 💿 Bootable x86_64/amd64 Desktop ISO
- ⚡ Linux kernel + systemd
- 🌐 NetworkManager networking
- 📁 Thunar file manager
- 🗜️ Archive support
- 📄 LibreOffice Writer and Calc
- 🎬 VLC and GStreamer multimedia support
- 📖 Evince document viewer
- 🔤 Liberation and Noto Color Emoji fonts
- 🛠️ Common Linux administration and developer utilities
- 🔐 NexOS security defaults
- 📦 Debian package ecosystem
- 🔧 Live-build based image pipeline
- 🧪 Automated ISO/boot validation
- 🧾 SHA-256 release checksums

## 💻 System Requirements

Recommended for the Desktop edition:

| Component | Recommended |
| --- | --- |
| Architecture | x86_64 / amd64 |
| RAM | 4 GB+ |
| Storage | 25 GB+ |
| USB | 4 GB+ for writing the ISO |
| Graphics | Standard Linux-compatible GPU |
| Boot | BIOS or UEFI |

The ISO is intended for x86_64/amd64 PCs and virtual machines.

## 🚀 Getting Started

### 1. Download

Download the latest desktop ISO from:

**[NexOS Releases](https://github.com/hunterkritik-byte/NexOS/releases)**

### 2. Write the ISO to USB

On Linux, identify your USB device carefully:

```sh
lsblk
```

Then write the ISO:

```sh
sudo dd if=NexOS-x86_64-v0.2.0.iso of=/dev/sdX bs=4M status=progress conv=fsync
```

Replace `/dev/sdX` with the correct USB device.

> **Warning:** `dd` permanently overwrites the selected device. Double-check the device before running it.

You can alternatively use a graphical USB imaging tool.

### 3. Boot NexOS

Insert the USB into the target PC and boot from it using the firmware boot menu.

For a safe first test, select the live environment and verify the desktop and hardware before installing.

## 🧪 Field Test

The v0.2.0 Desktop ISO passed the initial field test.

Test areas include:

- ISO generation
- x86_64 image generation
- Bootable live image
- Desktop environment
- Basic desktop startup
- Core system userspace
- Network stack
- Package integration

Hardware coverage will continue to expand with future releases.

## 🧰 Build NexOS Yourself

### Dependencies

On Debian/Ubuntu:

```sh
sudo apt update
sudo apt install live-build debootstrap qemu-system-x86 xorriso squashfs-tools
```

### Build the Desktop ISO

```sudo bash ./scripts/build-linux.sh```

The expected output is:

```
dist/NexOS-Linux-amd64.iso
dist/NexOS-Linux-amd64.iso.sha256
```

### Build a smaller command-line image

```sh
sudo bash ./scripts/build-linux.sh --no-desktop
```

### Test with QEMU

```sh
./scripts/smoke-test.sh dist/NexOS-Linux-amd64.iso
```

Do not install development images onto a production disk.

## 🏗️ Architecture

```
Firmware
   |
   +-- BIOS / UEFI
   |
GRUB / Debian Live
   |
Linux kernel
   |
systemd
   |
Debian userspace
   |
NexOS configuration + packages
   |
Xfce desktop
   |
Applications
```

## 📦 Editions

| Edition | Status | Goal |
| --- | --- | --- |
| **Desktop** | 🟢 Available | Friendly Xfce desktop for everyday use |
| **Core** | 🟡 Planned | Minimal command-line NexOS |
| **Developer** | 🟡 Planned | Desktop + development tooling |
| **Future** | 🔵 Planned | Additional architectures and deeper NexOS platform work |

## 🗺️ Roadmap

The project is focused on making NexOS a reliable daily-use desktop distribution.

### Current priorities

- Hardware compatibility testing
- Wi-Fi and Bluetooth validation
- Audio and USB testing
- Installer and recovery experience
- First-boot experience
- Release automation
- Documentation
- Hardware compatibility matrix
- Improved NexOS branding and desktop theme

See [docs/ROADMAP.md](docs/ROADMAP.md).

## 📚 Documentation

- [Linux Distribution Architecture](docs/LINUX-DISTRO.md)
- [Roadmap](docs/ROADMAP.md)
- [Original OS Architecture](docs/ARCHITECTURE.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)

## 🐛 Bug Reports & Security

For normal bugs and feature requests, please use the repository's GitHub Issues.

For security-sensitive vulnerabilities, follow the instructions in [SECURITY.md](SECURITY.md) and avoid publicly disclosing exploitable details before a fix is available.

## 🤝 Contributing

Contributions are welcome.

Before submitting changes:

1. Read [CONTRIBUTING.md](CONTRIBUTING.md).
2. Keep changes focused.
3. Test the affected build or component.
4. Document user-visible changes.
5. Do not include secrets or credentials.

## 📜 Release History

### v0.2.0 — First Desktop ISO

- First full NexOS Desktop distribution
- x86_64/amd64 target
- Approximately 2.54 GB ISO
- Xfce desktop
- Debian-based userspace
- Initial field test passed
- Release checksum support

**Full Changelog:** https://github.com/hunterkritik-byte/NexOS/commits/v0.2.0

### v0.1.0 — Early Kernel/Development Image

The original 4.5 MB development/kernel image.

**Full Changelog:** https://github.com/hunterkritik-byte/NexOS/commits/v0.1.0

## 📄 License

NexOS is released under the **Apache-2.0 License**.

## 👤 Maintainer

**Kritik Bhattarai**

Sponsorship & collaboration: **hunterkritik@gmail.com**

---

<p align="center">
  <strong>NexOS — Built by Kritik Bhattarai</strong>
</p>
