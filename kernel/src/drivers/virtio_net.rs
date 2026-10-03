use crate::net::device::NetworkDevice;

const QUEUE_SIZE: usize = 8;
const PACKET_SIZE: usize = 1536;

#[repr(C, align(16))]
struct Descriptor {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(C, align(4096))]
struct Queue {
    desc: [Descriptor; QUEUE_SIZE],
    avail: [u16; QUEUE_SIZE + 3],
    used: [u8; 4096],
}

pub struct VirtioNet {
    mac: [u8; 6],
    mmio_base: usize,
    rx_queue: Queue,
    tx_queue: Queue,
    rx_buffers: [[u8; PACKET_SIZE]; QUEUE_SIZE],
    tx_buffers: [[u8; PACKET_SIZE]; QUEUE_SIZE],
    rx_ready: bool,
    tx_ready: bool,
}

impl VirtioNet {
    pub const fn new(mmio_base: usize, mac: [u8; 6]) -> Self {
        Self {
            mac, mmio_base,
            rx_queue: Queue {
                desc: [Descriptor { addr:0,len:0,flags:0,next:0 }; QUEUE_SIZE],
                avail: [0; QUEUE_SIZE + 3], used: [0; 4096],
            },
            tx_queue: Queue {
                desc: [Descriptor { addr:0,len:0,flags:0,next:0 }; QUEUE_SIZE],
                avail: [0; QUEUE_SIZE + 3], used: [0; 4096],
            },
            rx_buffers: [[0; PACKET_SIZE]; QUEUE_SIZE],
            tx_buffers: [[0; PACKET_SIZE]; QUEUE_SIZE],
            rx_ready: false, tx_ready: false,
        }
    }

    pub unsafe fn initialize(&mut self) -> Result<(), &'static str> {
        if self.mmio_base == 0 { return Err("invalid VirtIO MMIO base"); }

        // This driver is intentionally explicit about its hardware boundary.
        // Full feature negotiation and queue programming must be performed
        // against the device's transport before enabling RX/TX.
        self.setup_descriptor_memory();
        Err("VirtIO transport negotiation is not yet connected")
    }

    fn setup_descriptor_memory(&mut self) {
        for i in 0..QUEUE_SIZE {
            self.rx_queue.desc[i].addr = self.rx_buffers[i].as_ptr() as u64;
            self.rx_queue.desc[i].len = PACKET_SIZE as u32;
            self.rx_queue.desc[i].flags = 2;
            self.tx_queue.desc[i].addr = self.tx_buffers[i].as_ptr() as u64;
        }
    }

    pub fn is_rx_ready(&self) -> bool { self.rx_ready }
    pub fn is_tx_ready(&self) -> bool { self.tx_ready }
}

impl NetworkDevice for VirtioNet {
    fn mac_address(&self) -> [u8; 6] { self.mac }

    fn transmit(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if !self.tx_ready { return Err("VirtIO TX queue is not ready"); }
        if packet.len() > PACKET_SIZE { return Err("packet too large"); }
        self.tx_buffers[0][..packet.len()].copy_from_slice(packet);
        Err("VirtIO TX submission is not connected to transport")
    }

    fn receive(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.rx_ready { return Err("VirtIO RX queue is not ready"); }
        Err("VirtIO RX completion is not connected to transport")
    }

    fn link_up(&self) -> bool { self.rx_ready && self.tx_ready }
}
