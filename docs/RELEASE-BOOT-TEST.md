# NexOS Boot Release Test

## BIOS

Boot the ISO with a legacy BIOS VM and verify:
1. GRUB appears.
2. NexOS live system boots.
3. LightDM appears.
4. Xfce starts automatically.
5. Terminal, file manager and Firefox launch.

## UEFI

Boot the same ISO through OVMF and verify the same sequence.

## Wired networking

Attach a virtual Ethernet adapter and verify:

```sh
nmcli device status
ip addr
ping -c 3 1.1.1.1
```

The test is passed when NetworkManager detects the interface and DHCP provides an address.

## Wi-Fi

Physical Wi-Fi testing remains hardware-specific. Verify the adapter is detected and document any required firmware package.

Do not mark the release as fully hardware-tested from a VM-only test.
