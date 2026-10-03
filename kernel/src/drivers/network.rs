use super::PciDevice;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NetworkKind {
    Ethernet,
    WirelessOrOther,
}

#[derive(Clone, Copy)]
pub struct NetworkController {
    pub pci: PciDevice,
    pub kind: NetworkKind,
    pub driver: &'static str,
}

impl NetworkController {
    pub const fn from_pci(device: PciDevice) -> Self {
        let kind = if device.subclass == 0x00 {
            NetworkKind::Ethernet
        } else {
            NetworkKind::WirelessOrOther
        };

        let driver = match device.vendor_id {
            0x8086 => "intel-network (probe-only)",
            0x10ec => "realtek-network (probe-only)",
            0x14e4 => "broadcom-network (probe-only)",
            0x168c => "qualcomm-atheros-network (probe-only)",
            0x1814 => "mediatek-ralink-network (probe-only)",
            _ => "generic-pci-network (probe-only)",
        };

        Self { pci: device, kind, driver }
    }
}
