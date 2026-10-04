pub mod device {
    use crate::x86_kernel::drivers::{NetworkKind, PciDevice};

    #[derive(Clone, Copy)]
    pub struct RegisteredPci { pub pci: PciDevice }

    pub struct NetworkRegistry {
        devices: [Option<RegisteredPci>; 8],
        count: usize,
    }

    impl NetworkRegistry {
        pub const fn new() -> Self { Self { devices: [None; 8], count: 0 } }

        pub fn register_pci(&mut self, pci: PciDevice) {
            if self.count < self.devices.len() {
                self.devices[self.count] = Some(RegisteredPci { pci });
                self.count += 1;
            }
        }

        pub fn len(&self) -> usize { self.count }

        pub fn describe(&self, index: usize) -> Option<(&'static str, NetworkKind, PciDevice)> {
            let entry = self.devices.get(index)?.as_ref()?;
            Some(("pci-probe", crate::x86_kernel::drivers::network::classify(entry.pci), entry.pci))
        }
    }
}
