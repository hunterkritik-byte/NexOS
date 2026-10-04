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
- [x] Add ISO checksum generation
- [x] Add GitHub Actions build validation
- [ ] Produce and QEMU-test a successful CI ISO
- [ ] Add signed release artifacts

### D1 — First usable desktop
- [ ] Boot reliably in BIOS and UEFI
- [ ] Xfce desktop starts automatically
- [ ] NetworkManager wired networking works
- [ ] Wi-Fi firmware/package policy documented
- [ ] Audio, Bluetooth and USB hardware smoke tests
- [ ] Calamares installer completes a VM installation
- [ ] First boot after installation succeeds
- [ ] NexOS welcome/setup application
- [ ] Custom NEXOS boot splash: "NEXOS — MADE BY KRITIK BHATTARAI"
- [ ] Default desktop wallpaper and theme
- [x] File manager, terminal, browser and settings package foundation
- [x] Wi-Fi, Bluetooth, audio and printing package foundation

### D2 — NexOS system layer
- [x] NexOS system CLI
- [x] Hardware diagnostics
- [ ] Update/recovery utility
- [x] NexOS configuration package
- [ ] First-party package repository
- [ ] Signed package metadata
- [ ] Release manifest and SBOM

### D3 — Security and reliability
- [ ] Secure defaults
- [ ] Automatic security update policy
- [ ] AppArmor profile set
- [ ] Firewall defaults
- [ ] Recovery environment
- [ ] Rollback/update recovery strategy
- [ ] Reproducible image verification

### D4 — Product polish
- [ ] Custom Plymouth/GRUB branding
- [ ] NexOS desktop theme and icon set
- [ ] First-run onboarding
- [ ] Software center/package UX
- [ ] Documentation site
- [ ] Hardware compatibility matrix

### D5 — Advanced platform
- [ ] ARM64 image
- [ ] Optional NexOS kernel configuration
- [ ] NexOS-specific low-level services where justified
- [ ] Developer SDK
- [ ] OEM/installer automation

## Research track

The existing Rust kernel remains experimental until it has independently demonstrated safe userspace isolation, scheduling, persistent storage, networking and hardware support. It must not be presented as the kernel used by NexOS Linux.
