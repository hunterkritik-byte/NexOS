# NexOS: path toward a general-purpose operating system

NexOS is an experimental x86_64 hobby OS. A feature is considered working only after it is integrated, tested, and its limitations are documented. A source module or successful compilation is not enough.

This plan deliberately separates foundations from end-user functionality. It is not a promise that every feature will land in one release.

## 1. Make boot and recovery reliable first

- Make formatting, workspace checks, kernel and userspace builds mandatory CI gates.
- Have QEMU tests capture serial logs and assert explicit boot milestones; a timeout alone must not count as a pass.
- Test BIOS and UEFI separately, and distinguish a booted kernel from a kernel that reached the interactive shell.
- Keep a known-good boot image and a documented recovery path.
- Publish checksums, build instructions, supported firmware requirements, and known limitations.

Acceptance: CI fails on a panic, missing expected boot marker, or failed command. Both firmware paths reach a documented milestone in repeatable QEMU runs.

## 2. Persistent storage

The current VFS is volatile. The persistent filesystem implementation in this branch is a small fixed-slot prototype, not a mature filesystem, and it is not wired into the boot path.

Next work:
1. Unit-test metadata encoding/decoding, duplicate names, invalid metadata, disk bounds, empty files, maximum-size files, and read/write round trips using MemoryDisk.
2. Fix and test ATA PIO readiness, LBA bounds, write completion/flush, and absent-device detection before using it.
3. Integrate a selected block device only after the driver passes tests; never auto-format an unrecognized disk.
4. Add explicit format/mount commands and a disposable test-disk workflow.
5. Add a recovery strategy and crash-consistency tests. Consider adopting a mature, well-specified filesystem format instead of growing an ad hoc format indefinitely.

Acceptance: files survive a reboot in QEMU; malformed metadata cannot trigger out-of-bounds access; a failed mount never silently reformats existing data.

## 3. A real userspace shell and process model

The current NexShell is a print-and-yield stub. The kernel shell is a separate diagnostic shell and must not be mistaken for the finished userspace shell.

Required work:
- Define a syscall ABI for terminal input/output, process exit, file operations, time, and basic process information.
- Validate user pointers and lengths before dereferencing them in the kernel.
- Add a terminal input queue and connect keyboard input to a blocking/read syscall.
- Implement shell parsing with quoting, arguments, and predictable error codes.
- Add cd, directory-aware ls, cat, file creation/writes, and built-in help/status.
- Implement process creation/exit/wait, context switching, timer-driven preemption, address-space isolation, and cleanup.
- Add core utilities as separate user programs and an init/service lifecycle.

Acceptance: the user shell accepts keyboard input, launches at least one separate program, reaps its exit status, and cannot read or write another process's memory.

## 4. Wired networking before wireless networking

Do not start with Wi-Fi or hotspot UI. First support one known virtual NIC end to end.

- Choose a concrete target (for example, a specific QEMU NIC model) and document the PCI IDs and transport.
- Implement DMA-safe buffers and queue/ring ownership, including physical address translation and memory barriers.
- Test RX/TX, malformed packets, timeouts, reset, and device absence.
- Connect Ethernet, ARP, IPv4, DHCP, DNS, UDP/TCP, and sockets to that active NIC.
- Test using a private QEMU user-mode network or isolated test network.

Acceptance: NexOS obtains a DHCP lease and completes a documented network test without unsafe DMA assumptions. PCI discovery alone is not network support.

## 5. Wi-Fi and hotspot

After a stable wired stack:
- Select and document one specific Wi-Fi chipset and supported firmware.
- Implement its bus transport, firmware loading, scan, association, authentication, and key handling.
- Provide a network configuration service and diagnostics.
- Only advertise hotspot support after AP mode, authentication, DHCP, routing/NAT, and client isolation have been tested.

A generic command named wifi or hotspot is not a driver and must not claim the radio is enabled.

## 6. Bluetooth

Select one USB or PCIe controller and implement its transport, HCI command/event handling, reset and discovery, then the required L2CAP and pairing/security behavior. Add supported-profile documentation and test against real devices. Until this passes, Bluetooth remains unsupported.

## 7. USB input, graphics, and applications

- Implement USB host-controller support (start with one QEMU-supported controller), enumeration, and USB HID keyboard/mouse.
- Build on the framebuffer code with a stable graphics API, font/text rendering, compositor, window manager, and desktop shell.
- Add a file manager, settings, terminal application, and a small application/runtime packaging format.
- Keep a text/serial recovery console available if graphics or USB fails.

Acceptance: keyboard and mouse work through USB in QEMU, a graphical session starts repeatably, and the user can launch and close a basic application.

## 8. Installer and distribution

- Choose a supported installation target and partition/filesystem layout.
- Never overwrite a disk without explicit confirmation and a clear target-device display.
- Provide install media, bootloader setup, an uninstall/recovery path, and upgrade/rollback rules.
- Test install, first boot, update, failed update recovery, and removal in disposable virtual disks.
- Publish hardware compatibility, firmware requirements, known issues, checksums, and reproducible build instructions.

## Release policy

Do not label NexOS ready for daily use until storage persistence, input, process isolation, networking claims, installation/recovery, and the relevant hardware support have end-to-end tests. Until then, describe it as an experimental OS for QEMU and development hardware.
