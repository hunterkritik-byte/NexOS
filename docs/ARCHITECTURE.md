# NexOS Architecture

NexOS has two explicit engineering tracks.

## Primary: NexOS Linux

NexOS Linux is a Debian-based Linux distribution. The Linux kernel supplies hardware compatibility and the core kernel ABI; NexOS owns the distribution layer.

```
Firmware
  -> bootloader / Debian Live
  -> Linux kernel
  -> systemd
  -> Debian userspace
  -> NexOS packages and configuration
  -> NexOS desktop and applications
```

This is the path intended to become the usable, installable NexOS product.

## Research: NexKernel

The repository also contains the original from-scratch Rust kernel work:

```
Firmware -> Rust bootloader -> NexKernel -> experimental userspace
```

NexKernel is not currently the kernel of NexOS Linux. It is retained for low-level operating-system research, experiments, and future evaluation.

## Design principle

Prefer upstream Linux/Debian components when they provide mature hardware support and security updates. Build NexOS-specific functionality above them unless there is a concrete reason to fork or replace a subsystem.

A subsystem is considered production-ready only after integration, testing, recovery behavior, and documented hardware compatibility.
