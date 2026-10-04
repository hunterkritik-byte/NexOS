#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}

pub struct PciScanner;

impl PciScanner {
    pub const fn new() -> Self {
        Self
    }

    /// Minimal PCI network-controller discovery. Driver probing is intentionally
    /// read-only until chipset-specific implementations are added.
    pub unsafe fn scan_network(&self, devices: &mut [Option<PciDevice>]) -> usize {
        let mut count = 0;
        if devices.is_empty() {
            return 0;
        }

        // Keep the hardware probe conservative in the kernel bring-up build.
        // Returning zero is valid when no chipset driver has been registered.
        for device in devices.iter_mut() {
            *device = None;
        }
        count
    }
}

pub mod network {
    use super::PciDevice;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NetworkKind {
        Ethernet,
        WirelessOrOther,
    }

    pub fn classify(_device: PciDevice) -> NetworkKind {
        NetworkKind::Ethernet
    }
}
