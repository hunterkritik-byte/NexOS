# NexOS Boot, Desktop and Networking Policy

## BIOS and UEFI

NexOS targets x86_64 PCs and builds a hybrid ISO. The image includes the GRUB BIOS and UEFI boot components and an EFI bootloader.

Release testing must cover:
- Legacy BIOS boot in QEMU
- UEFI boot in QEMU/OVMF
- Live desktop startup
- Installation to a fresh UEFI virtual disk
- Installation to a fresh BIOS/MBR virtual disk

A release is not considered boot-ready until both BIOS and UEFI tests pass.

## Xfce desktop

LightDM is enabled and configured for the NexOS desktop session. The live image uses the `nexos` session user and starts Xfce automatically.

For an installed system, the installer-created user should receive the normal graphical login experience.

## Wired networking

NetworkManager is the system network manager. Wired Ethernet should be plug-and-play when the kernel exposes a supported NIC.

Check:

```sh
nmcli general status
nmcli device status
ip link
```

Expected: an Ethernet device appears and obtains an address from DHCP when connected to a normal network.

## Wi-Fi firmware policy

NexOS follows Debian's firmware model. The image enables the `non-free-firmware` archive component and includes commonly useful Debian firmware packages where legally/distributably available.

NexOS does **not** bundle every vendor firmware blob. Some adapters require a hardware-specific Debian firmware package, and some vendor firmware may have separate licensing requirements.

When Wi-Fi is missing:

```sh
nmcli device
rfkill list
dmesg | grep -Ei 'firmware|wifi|wlan|iwlwifi|ath|rtw|brcm'
```

Then identify the hardware with:

```sh
lspci -nnk
lsusb
```

Install the matching Debian firmware package when available and reboot.

This policy intentionally avoids promising universal Wi-Fi support. Compatibility depends on the physical adapter, kernel driver and available firmware.
