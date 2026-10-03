# NexOS Networking and Wireless Support

## Current state

The current boot path scans PCI configuration space and reports network-class devices. This is discovery only: it does not attach a working driver or establish a network connection. The existing VirtIO network implementation deliberately returns an error before enabling DMA because it does not yet have safe physical-address mapping and complete queue setup.

The `wifi`, `hotspot`, and `bluetooth` shell commands are diagnostic status commands. They report that these features are unavailable; they do not enable radios or create a hotspot.

## Why a generic “Wi-Fi on” switch is not enough

Wi-Fi depends on the exact chipset, its bus (PCIe or USB), firmware, DMA/interrupt setup, and regulatory/channel configuration. A usable client connection also needs 802.11 scanning, association, authentication, key handling, and integration with the IP stack.

A hotspot requires more than joining a Wi-Fi network: the selected chipset must support access-point mode, and NexOS needs authentication/key management, DHCP service, address configuration, and usually routing/NAT to share an upstream connection.

Bluetooth is a separate radio and protocol stack. Support requires a known controller and transport (for example, USB HCI), HCI command/event handling, device discovery, pairing/security, and the relevant higher-level profiles. A Wi-Fi adapter does not automatically provide a Bluetooth controller even when the laptop markets them as one combo card.

## Implementation plan

1. **Choose real hardware targets.** Record PCI/USB IDs and select one chipset/controller for which programming documentation and firmware redistribution terms are available.
2. **Make wired networking reliable first.** Finish one NIC driver, including DMA-safe buffers, queue/descriptor ownership, interrupts or polling, link state, and end-to-end QEMU tests.
3. **Build a device/firmware layer.** Validate device IDs and firmware image lengths, keep DMA buffers physically addressable, and make failure states explicit.
4. **Add Wi-Fi client support.** Implement the selected chipset's transport and firmware protocol, then 802.11 management/security and integration with DHCP/DNS.
5. **Add hotspot mode only after client mode.** Verify AP-mode capability on the selected card, then implement authentication, DHCP, and routing. Do not claim internet sharing until clients can connect and pass traffic.
6. **Add Bluetooth for a named controller.** Implement transport and HCI first, then discovery, pairing, and only the profiles NexOS intends to support.
7. **Publish a compatibility matrix.** State tested device IDs, firmware requirements, supported modes, and known limitations. “Detected” must not be described as “supported.”

## Safety and release criteria

- Never enable DMA with kernel virtual addresses passed as if they were physical addresses.
- Validate descriptor lengths, packet lengths, queue indices, and all device-provided values.
- Do not log Wi-Fi passwords, pairing secrets, or key material.
- Test malformed packets and device-reset/error paths.
- Mark a device supported only after repeatable tests on QEMU where applicable and the named physical hardware.
