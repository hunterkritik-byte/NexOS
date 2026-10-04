pub trait NetworkDevice {
    fn mac_address(&self) -> [u8; 6];
    fn transmit(&mut self, packet: &[u8]) -> Result<(), &'static str>;
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
    fn link_up(&self) -> bool;
}

#[derive(Clone, Copy)]
pub struct RegisteredPci {
    pub pci: crate::x86_kernel::drivers::PciDevice,
}

pub struct NetworkRegistry {
    devices: [Option<RegisteredPci>; 8],
    count: usize,
}

impl NetworkRegistry {
    pub const fn new() -> Self {
        Self {
            devices: [None; 8],
            count: 0,
        }
    }

    pub fn register_pci(&mut self, pci: crate::x86_kernel::drivers::PciDevice) {
        if self.count < self.devices.len() {
            self.devices[self.count] = Some(RegisteredPci { pci });
            self.count += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn describe(
        &self,
        index: usize,
    ) -> Option<(
        &'static str,
        crate::x86_kernel::drivers::network::NetworkKind,
        crate::x86_kernel::drivers::PciDevice,
    )> {
        let entry = self.devices.get(index)?.as_ref()?;
        Some(("pci-probe", crate::x86_kernel::drivers::network::classify(entry.pci), entry.pci))
    }
}
