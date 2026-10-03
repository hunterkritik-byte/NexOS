use crate::drivers::network::{NetworkController, NetworkKind};
use crate::drivers::PciDevice;

pub struct NetworkRegistry {
    entries: [Option<NetworkController>; 8],
    len: usize,
}

impl NetworkRegistry {
    pub const fn new() -> Self {
        Self {
            entries: [None; 8],
            len: 0,
        }
    }

    pub fn register_pci(&mut self, device: PciDevice) {
        if self.len < self.entries.len() {
            self.entries[self.len] = Some(NetworkController::from_pci(device));
            self.len += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn describe(&self, index: usize) -> Option<(&'static str, NetworkKind, PciDevice)> {
        self.entries.get(index).and_then(|entry| {
            entry.map(|controller| (controller.driver, controller.kind, controller.pci))
        })
    }
}
