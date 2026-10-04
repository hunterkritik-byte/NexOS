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
