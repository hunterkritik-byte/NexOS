# NexOS Installer, Recovery and First Boot

## Installer

NexOS uses **Calamares** for graphical installation. The live desktop should provide a launcher for Calamares; the final ISO must be tested in a disposable virtual disk before release.

The installer must be tested for:
- UEFI boot
- GPT partitioning
- root filesystem creation
- bootloader installation
- user creation
- reboot into the installed system

## Recovery

NexOS provides a safe command-line recovery helper:

```sh
nexos-recovery help
nexos-recovery status
nexos-recovery journal
nexos-recovery network
sudo nexos-recovery repair
```

`repair` changes the system only after explicit invocation and runs package configuration repair plus initramfs regeneration.

## First boot

The first-boot service runs once and creates a small NexOS welcome/help file. It is idempotent and does not silently modify networking, partitions or user data.

## Release gate

Do not call the installer production-ready until a clean VM has been:
1. booted from the live ISO;
2. installed to a fresh virtual disk;
3. rebooted without the ISO;
4. logged into the desktop;
5. verified for networking, audio, Bluetooth and storage;
6. verified for the recovery commands.
