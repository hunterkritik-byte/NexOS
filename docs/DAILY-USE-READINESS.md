# NexOS Daily-Use Readiness

This document defines what NexOS needs before it is presented as a daily-use desktop distribution.

## User experience

- Xfce desktop and LightDM
- Thunar file manager
- Firefox ESR
- Terminal and standard Linux CLI
- NetworkManager
- Bluetooth/BlueZ
- PipeWire/WirePlumber audio
- CUPS printing
- UDisks/GVFS removable storage
- NexOS Settings and System Tools
- NexOS Recovery Tools

## Security defaults

The desktop image includes AppArmor, UFW, unattended security upgrades, audit tooling and package-change notifications.

Default policy:
- deny unsolicited incoming connections
- allow outbound connections
- keep AppArmor profiles enabled where available
- receive Debian security updates through the normal APT security channels
- never claim that automatic updates replace user backups

Check the security state:

```sh
nexos-maintenance security
```

## Maintenance

```sh
nexos-maintenance updates
nexos-maintenance disk
nexos-maintenance status
```

## Release blockers

The following require a real ISO/runtime test before release:
- BIOS and UEFI boot
- live desktop startup
- installation and first reboot
- wired networking
- Wi-Fi connection
- Bluetooth pairing
- speaker playback and microphone capture
- physical USB insertion/removal
- suspend/resume
- shutdown/reboot
- display/graphics behavior
- printer discovery/printing
- browser launch and network access
- installer recovery path

A CI package/configuration test is not a substitute for these physical tests.


## Essential daily utilities

The desktop package profile includes archive integration and common productivity/media applications:
- zip, unzip, p7zip-full
- Thunar archive integration
- Evince PDF reader
- LibreOffice Writer and Calc
- VLC media player

## Web media and fonts

The image includes common GStreamer good/bad/ugly plugin sets plus Liberation and Noto Color Emoji fonts. These improve browser/media compatibility and document/text rendering. Proprietary media services may still require their own DRM/content policies.

## Laptop experience

TLP/TLP-RDW and Xfce Power Manager are included for power and lid/sleep management. Redshift is included as an optional night-light utility.

## Package update safety

apt-listbugs and apt-listchanges are included so package changes can surface known bug and changelog information before or during upgrades. They do not guarantee that an update is safe; users should keep backups and use the NexOS recovery path if a package transition fails.
