# NexOS Linux distribution architecture

## Direction

NexOS Linux is the practical product direction of this repository.

The previous project was a from-scratch Rust kernel. That work remains useful for OS research, but it should not block a usable Linux distribution. NexOS Linux therefore uses the Linux kernel and Debian userspace as its compatibility foundation while owning the distribution layer.

## Layering

Firmware
-> GRUB / Debian Live boot infrastructure
-> Linux kernel
-> systemd
-> Debian base userspace
-> NexOS packages/configuration
-> NexOS desktop and system tools
-> applications

## What makes it NexOS

The distribution must not be just a renamed Debian ISO. NexOS-owned work will progressively include:

1. NexOS branding and release metadata.
2. A curated package set and default configuration.
3. NexOS system utilities and CLI.
4. A first-party repository for NexOS packages.
5. Installer branding and automated hardware checks.
6. Security baseline and update policy.
7. A desktop experience and default applications.
8. Release engineering, signed artifacts and reproducible builds.
9. Hardware compatibility and recovery tooling.
10. Optional NexOS-specific kernel configuration once the userspace foundation is stable.

## Compatibility rule

Use Debian packages where practical instead of forking large upstream projects. Fork only when NexOS has a concrete product requirement. This keeps security updates maintainable.

## Editions

### Core
Minimal command-line Linux for servers, development and recovery.

### Desktop
Xfce-based desktop focused on broad hardware support and modest resource use.

### Developer
Desktop plus compilers, debuggers, containers, Git and NexOS development tools.

### Future
A custom desktop shell and a NexOS-specific kernel configuration can be added without changing the base distribution architecture.

## Current milestone

Milestone 0.1 establishes a real Debian-based NexOS live image build. It does not claim that the distribution is production-ready.
