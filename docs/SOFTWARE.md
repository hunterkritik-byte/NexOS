# NexOS Software

NexOS uses Debian's package system rather than a proprietary package format.

## Terminal package management

```sh
sudo apt update
sudo apt install PACKAGE
sudo apt remove PACKAGE
apt search KEYWORD
apt list --upgradable
```

## NexOS Software entry

The desktop includes a **NexOS Software** launcher for package maintenance.

A dedicated graphical software center remains a future product component; the current UX intentionally uses Debian APT instead of pretending to provide a full software store.
