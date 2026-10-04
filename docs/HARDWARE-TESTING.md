# NexOS Hardware Testing

These checks are intended for the live system and the installed system.

## Networking / Wi-Fi

```sh
nmcli general status
nmcli device status
nmcli radio wifi
nmcli device wifi list
```

Expected: NetworkManager is running and the wireless adapter is visible. Connect through the desktop NetworkManager applet or `nmcli`.

## Bluetooth

```sh
systemctl status bluetooth
bluetoothctl show
bluetoothctl power on
bluetoothctl scan on
```

Expected: the controller is detected and can discover a test device.

## Audio

```sh
systemctl --user status pipewire wireplumber pipewire-pulse
wpctl status
```

Expected: PipeWire and WirePlumber are active and playback/capture devices are visible.

## Storage

```sh
lsblk -o NAME,SIZE,TYPE,FSTYPE,MOUNTPOINTS
udisksctl status
```

Expected: internal and removable disks are enumerated and removable media can be mounted from the file manager.

## One-command diagnostic

```sh
sudo /usr/share/nexos/hardware-check.sh
```

Hardware results depend on the target machine and available Linux firmware/drivers. A missing physical device is not treated as an image-build failure.


## Extended CI hardware validation

The `.github/workflows/hardware-extended.yml` workflow checks the Linux interfaces used by Wi-Fi, Bluetooth, audio and USB.

A GitHub-hosted runner cannot honestly pass these physical tests:
- Bluetooth pairing with a real device
- Speaker playback through a physical sound device
- Microphone capture from a physical microphone
- USB physical hotplug/insertion
- Wi-Fi association using a physical adapter

Those require a NexOS x86_64 machine with the hardware attached, preferably as a self-hosted GitHub Actions runner or a manual ISO test machine.
