# NexOS Roadmap

NexOS now has two tracks:

- **NexOS Linux** — the primary product: a practical Linux distribution based on Debian.
- **NexKernel research** — the original from-scratch Rust kernel project, kept for OS research and experimentation.

## Distribution track

### D0 — Linux foundation
- [x] Choose Debian 13 trixie as the Linux base
- [x] Add live-build based ISO pipeline
- [x] Add NexOS release metadata and branding
- [x] Add core/desktop package profile
- [x] Add ISO checksum generation (pipeline configuration)
- [x] Add GitHub Actions build validation
- [ ] Produce and QEMU-test a successful CI ISO (blocked until CI build is fixed)
- [ ] Add signed release artifacts

### D1 — First usable desktop
- [x] Boot configuration for BIOS and UEFI; release validation pending final ISO runtime tests
- [x] LightDM configured for automatic Xfce desktop startup
- [x] NetworkManager wired networking configuration and release test
- [x] Wi-Fi firmware/package policy documented
- [x] CI audio, Bluetooth and USB interface smoke tests; physical hardware validation remains a release gate
- [x] Calamares installer launcher and installation documentation
- [x] First-boot service and first-boot documentation
- [x] NexOS first-boot welcome/help experience
- [x] Custom NEXOS boot splash: "NEXOS — MADE BY KRITIK BHATTARAI"
- [x] Default desktop wallpaper and theme
- [x] File manager, terminal, browser and settings package foundation
- [x] Wi-Fi, Bluetooth, audio and printing package foundation

### D2 — NexOS system layer
- [x] NexOS system CLI
- [x] Hardware diagnostics
- [x] NexOS update/recovery utility
- [x] NexOS configuration package
- [ ] First-party package repository
- [ ] Signed package metadata
- [x] Release manifest; SBOM generation remains a release-pipeline task

### D3 — Security and reliability
- [x] Secure defaults
- [x] Automatic security update policy
- [x] AppArmor profile set foundation
- [x] Firewall defaults
- [x] NexOS recovery tools
- [ ] Rollback/update recovery strategy
- [ ] Reproducible image verification

### D4 — Product polish
- [x] Custom Plymouth NexOS branding
- [x] NexOS desktop theme and icon set
- [x] First-run welcome/help foundation
- [x] Software/package UX (APT-based foundation; dedicated GUI store remains future work)
- [x] Documentation site foundation
- [x] Hardware compatibility matrix

### D5 — Advanced platform
- [ ] x64 image
- [ ] Optional NexOS kernel configuration
- [ ] NexOS-specific low-level services where justified
- [ ] Developer SDK
- [ ] OEM/installer automation

## Research track

The existing Rust kernel remains experimental until it has independently demonstrated safe userspace isolation, scheduling, persistent storage, networking and hardware support. It must not be presented as the kernel used by NexOS Linux.
