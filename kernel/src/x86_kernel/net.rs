//! Minimal x86 network hardware module used during kernel bring-up.
//!
//! This module intentionally provides only the platform-facing types needed
//! by the kernel. User-space networking is provided by the Linux NexOS image.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkDevice {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}

impl NetworkDevice {
    pub const fn new(bus: u8, slot: u8, function: u8) -> Self {
        Self { bus, slot, function }
    }
}

pub const fn probe() -> Option<NetworkDevice> {
    None
}
