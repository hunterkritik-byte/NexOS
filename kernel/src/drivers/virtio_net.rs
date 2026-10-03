use crate::net::device::NetworkDevice;

pub struct VirtioNet {
    mac: [u8; 6],
    ready: bool,
}

impl VirtioNet {
    pub const fn new(mac: [u8; 6]) -> Self { Self { mac, ready: false } }

    pub fn initialize(&mut self) -> Result<(), &'static str> {
        // Hardware-specific VirtIO queue/DMA setup belongs here.
        // Do not claim readiness until negotiated features and queues exist.
        self.ready = true;
        Ok(())
    }
}

impl NetworkDevice for VirtioNet {
    fn mac_address(&self) -> [u8; 6] { self.mac }

    fn transmit(&mut self, _packet: &[u8]) -> Result<(), &'static str> {
        if !self.ready { return Err("VirtIO NIC is not initialized"); }
        Err("VirtIO TX queue is not connected")
    }

    fn receive(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.ready { return Err("VirtIO NIC is not initialized"); }
        Err("VirtIO RX queue is not connected")
    }

    fn link_up(&self) -> bool { self.ready }
}
