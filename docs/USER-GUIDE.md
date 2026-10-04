# NexOS Linux User Guide

NexOS Desktop is designed as a normal graphical Linux desktop.

## Daily use

- **File manager:** Thunar
- **Browser:** Firefox ESR
- **Terminal:** Xfce Terminal
- **Network:** NetworkManager
- **Wi-Fi:** NetworkManager applet
- **Bluetooth:** Blueman
- **Audio:** PipeWire + WirePlumber
- **Printing:** CUPS
- **Installer:** Calamares

## NexOS CLI

Open a terminal and run:

```sh
nexos help
nexos info
nexos doctor
nexos network
nexos bluetooth
nexos audio
```

For updates:

```sh
nexos update
```

## Hardware support

NexOS targets x86_64/amd64 PCs. Hardware support depends on Linux kernel drivers and available firmware. Wi-Fi, Bluetooth, GPU and audio devices should be tested on the target machine.

## Installation

The final NexOS ISO will provide a live desktop and Calamares installation workflow. Installation must be tested in both UEFI and BIOS environments before a release is declared stable.

## Recovery

From a TTY or terminal, inspect:

```sh
nexos doctor
systemctl --failed
journalctl -b -p warning
```
