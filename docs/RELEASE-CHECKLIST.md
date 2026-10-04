# NexOS Daily-Use Release Checklist

## Build
- [ ] x86_64 ISO contains a real live filesystem
- [ ] ISO is larger than the tiny metadata-only artifact class
- [ ] `file`, ISO metadata and `live/filesystem.squashfs` verified
- [ ] SHA-256 generated
- [ ] SHA-512 generated
- [ ] Build commit/version recorded
- [ ] SBOM and release manifest generated

## Boot
- [ ] BIOS boot
- [ ] UEFI boot
- [ ] Plymouth branding
- [ ] LightDM/Xfce startup

## Desktop
- [ ] File manager
- [ ] Terminal
- [ ] Browser
- [ ] Settings
- [ ] Software/package workflow
- [ ] Wallpaper/theme
- [ ] Shutdown/reboot

## Hardware
- [ ] Ethernet
- [ ] Wi-Fi + firmware
- [ ] Bluetooth pairing
- [ ] Audio playback
- [ ] Microphone capture
- [ ] USB hotplug
- [ ] Storage mounting
- [ ] Printer
- [ ] Suspend/resume
- [ ] Display/GPU

## Security
- [ ] AppArmor active
- [ ] Firewall active
- [ ] Automatic security update policy verified
- [ ] Recovery procedure verified

## Installation
- [ ] Calamares installation
- [ ] User creation
- [ ] Installed-system boot
- [ ] First boot
- [ ] Recovery tools

Do not publish a release until all applicable runtime checks are completed.
