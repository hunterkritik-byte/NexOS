# NexOS Settings & System Tools

## Settings

The desktop menu contains **NexOS Settings**, which opens the Xfce settings manager.

From a terminal:

```sh
nexos-settings
```

## System tools

Run:

```sh
nexos-system help
```

Available modules:

- `info` — kernel, OS and architecture information
- `doctor` — hardware and failed-service diagnostics
- `network` — NetworkManager status
- `wifi` — Wi-Fi radio and nearby networks
- `bluetooth` — Bluetooth controller/service status
- `audio` — PipeWire/WirePlumber devices and services
- `storage` — disks, filesystems and UDisks
- `updates` — refresh package metadata and show updates
- `clean` — clean the APT package cache
- `settings` — open the graphical settings manager

The tools intentionally use the normal Linux services underneath rather than inventing a parallel hardware stack.
