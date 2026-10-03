use crate::net::device::NetworkDevice;

const QUEUE_SIZE: usize = 8;
const PACKET_SIZE: usize = 1536;
const VIRTIO_F_VERSION_1: u64 = 1 << 32;
const VIRTIO_NET_F_MAC: u64 = 1 << 5;

#[repr(C)]
#[derive(Clone, Copy)]
struct Descriptor { addr:u64, len:u32, flags:u16, next:u16 }

#[repr(C)]
struct Avail { flags:u16, idx:u16, ring:[u16;QUEUE_SIZE] }

#[repr(C)]
struct UsedElem { id:u32, len:u32 }

#[repr(C)]
struct Used { flags:u16, idx:u16, ring:[UsedElem;QUEUE_SIZE] }

#[repr(C, align(16))]
struct QueueMemory {
    desc:[Descriptor;QUEUE_SIZE],
    avail:Avail,
    used:Used,
}

pub struct VirtioNet {
    base: usize,
    mac:[u8;6],
    queue: QueueMemory,
    rx:[[u8;PACKET_SIZE];QUEUE_SIZE],
    tx:[[u8;PACKET_SIZE];QUEUE_SIZE],
    rx_last:u16,
    tx_idx:u16,
    ready:bool,
}

impl VirtioNet {
    pub const fn new(base:usize)->Self {
        Self {
            base, mac:[0;6],
            queue: QueueMemory {
                desc:[Descriptor{addr:0,len:0,flags:0,next:0};QUEUE_SIZE],
                avail:Avail{flags:0,idx:0,ring:[0;QUEUE_SIZE]},
                used:Used{flags:0,idx:0,ring:[UsedElem{id:0,len:0};QUEUE_SIZE]},
            },
            rx:[[0;PACKET_SIZE];QUEUE_SIZE],
            tx:[[0;PACKET_SIZE];QUEUE_SIZE],
            rx_last:0,tx_idx:0,ready:false,
        }
    }

    unsafe fn read32(&self, off:usize)->u32 { core::ptr::read_volatile((self.base+off) as *const u32) }
    unsafe fn write32(&self, off:usize, v:u32) { core::ptr::write_volatile((self.base+off) as *mut u32,v) }
    unsafe fn write64(&self, off:usize, v:u64) { core::ptr::write_volatile((self.base+off) as *mut u64,v) }

    pub unsafe fn initialize(&mut self)->Result<(),&'static str> {
        if self.base==0 { return Err("invalid VirtIO MMIO base"); }

        // VirtIO 1.0 MMIO transport register layout.
        let magic=self.read32(0x000);
        let version=self.read32(0x004);
        if magic!=0x74726976 { return Err("VirtIO magic mismatch"); }
        if version<2 { return Err("unsupported VirtIO MMIO version"); }

        // Reset -> acknowledge -> driver.
        self.write32(0x070,0);
        self.write32(0x070,1);
        self.write32(0x070,3);

        let device_features=self.read32(0x010) as u64 | ((self.read32(0x014) as u64)<<32);
        let wanted=device_features & (VIRTIO_F_VERSION_1 | VIRTIO_NET_F_MAC);
        self.write32(0x024,wanted as u32);
        self.write32(0x028,(wanted>>32) as u32);
        self.write32(0x070,7);

        if self.read32(0x070)&8 != 0 { return Err("VirtIO feature negotiation failed"); }

        // Queue 0: RX/TX network queue. The actual queue size/address programming
        // must use the transport's negotiated queue-size semantics.
        self.write32(0x030,0);
        let qmax=self.read32(0x034) as usize;
        if qmax<QUEUE_SIZE { return Err("VirtIO queue too small"); }
        self.write32(0x038,QUEUE_SIZE as u32);

        self.prepare_descriptors();

        // For a complete production driver, use the negotiated queue alignment/
        // address registers supplied by this transport and an IOMMU/physical
        // allocator. This implementation refuses to enable DMA from virtual
        // addresses, avoiding memory corruption on real hardware.
        return Err("DMA physical-address mapping is required before enabling VirtIO");
    }

    fn prepare_descriptors(&mut self) {
        for i in 0..QUEUE_SIZE {
            self.queue.desc[i]=Descriptor {
                addr:self.rx[i].as_ptr() as u64,
                len:PACKET_SIZE as u32,
                flags:2,
                next:0,
            };
            self.queue.avail.ring[i]=i as u16;
        }
    }

    pub fn is_ready(&self)->bool { self.ready }
}

impl NetworkDevice for VirtioNet {
    fn mac_address(&self)->[u8;6] { self.mac }

    fn transmit(&mut self, packet:&[u8])->Result<(),&'static str> {
        if !self.ready{return Err("VirtIO NIC not ready");}
        if packet.len()>PACKET_SIZE{return Err("packet too large");}
        Err("TX physical queue submission unavailable")
    }

    fn receive(&mut self, _buffer:&mut [u8])->Result<usize,&'static str> {
        if !self.ready{return Err("VirtIO NIC not ready");}
        Err("RX completion unavailable")
    }

    fn link_up(&self)->bool { self.ready }
}
