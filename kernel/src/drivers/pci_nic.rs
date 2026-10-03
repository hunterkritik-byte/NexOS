use super::pci::PciDevice;

pub trait PciNicDriver {
    fn matches(device:&PciDevice)->bool where Self:Sized;
    fn initialize(&mut self)->Result<(),&'static str>;
}

pub struct GenericPciNic {
    pub device:PciDevice,
    initialized:bool,
}

impl GenericPciNic {
    pub const fn new(device:PciDevice)->Self { Self{device,initialized:false} }
}

impl PciNicDriver for GenericPciNic {
    fn matches(device:&PciDevice)->bool { device.class==0x02 }
    fn initialize(&mut self)->Result<(),&'static str> {
        // Generic PCI discovery is real; packet DMA is deliberately not claimed
        // until a vendor-specific register/DMA implementation is selected.
        Err("no vendor-specific PCI NIC driver selected")
    }
}
