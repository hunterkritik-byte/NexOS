# NexOS Hardware Compatibility Matrix

NexOS targets **x86_64 / amd64 PCs**.

| Component | Target | Validation status |
|---|---|---|
| CPU | x86_64 Intel/AMD | Configuration ready; final ISO test pending |
| BIOS boot | Legacy PC BIOS | Configuration ready; final ISO test pending |
| UEFI boot | UEFI x86_64 | Configuration ready; final ISO test pending |
| Wired Ethernet | Linux-supported NIC | NetworkManager configured; hardware test pending |
| Wi-Fi | Linux-supported adapter + available firmware | Policy documented; hardware matrix test pending |
| Bluetooth | Linux-supported controller | BlueZ/Blueman configured; hardware test pending |
| Audio | ALSA/PipeWire-supported device | PipeWire configured; hardware test pending |
| Storage | SATA/NVMe/USB devices with kernel support | UDisks configured; hardware test pending |
| GPU | Linux-supported Intel/AMD/NVIDIA hardware | Driver-specific; hardware test pending |

## Reporting a device

Record:

```sh
uname -a
lspci -nnk
lsusb
rfkill list
nmcli device
wpctl status
lsblk -f
```

Include the exact device model, kernel version and NexOS release when reporting compatibility.
