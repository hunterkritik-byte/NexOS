#[derive(Clone, Copy)]
pub struct PciDevice {
    pub bus:u8,pub device:u8,pub function:u8,
    pub vendor_id:u16,pub device_id:u16,
    pub class:u8,pub subclass:u8,pub prog_if:u8,
    pub bars:[u32;6],
}

unsafe fn address(bus:u8,dev:u8,fun:u8,off:u8)->u32 {
    0x8000_0000|((bus as u32)<<16)|((dev as u32)<<11)|((fun as u32)<<8)|((off as u32)&0xfc)
}

pub unsafe fn config_read32(bus:u8,dev:u8,fun:u8,off:u8)->u32 {
    use x86_64::instructions::port::Port;
    let mut a=Port::<u32>::new(0xcf8);
    let mut d=Port::<u32>::new(0xcfc);
    a.write(address(bus,dev,fun,off)); d.read()
}

pub unsafe fn scan<F:FnMut(PciDevice)>(mut f:F) {
    for bus in 0..=255 { for dev in 0..32 {
        let h=config_read32(bus,dev,0,0);
        if h as u16==0xffff {continue}
        let funcs=if ((config_read32(bus,dev,0,0x0c)>>16)&0x80)!=0 {8} else {1};
        for fun in 0..funcs {
            let id=config_read32(bus,dev,fun,0);
            if id as u16==0xffff {continue}
            let class=config_read32(bus,dev,fun,8);
            let mut bars=[0;6];
            for i in 0..6 {bars[i]=config_read32(bus,dev,fun,0x10+i*4);}
            f(PciDevice{bus,device:dev,function:fun,vendor_id:id as u16,device_id:(id>>16) as u16,
                class:(class>>24) as u8,subclass:(class>>16) as u8,prog_if:(class>>8) as u8,bars});
        }
    }}
}

pub fn is_network_controller(d:PciDevice)->bool { d.class==0x02 }

/// Minimal PCI configuration-space scanner used during kernel bring-up.
/// This discovers network-class devices; it does not initialize their drivers.
pub struct PciScanner;

impl PciScanner {
    pub const fn new() -> Self {
        Self
    }

    /// Fill the caller-provided array with discovered network controllers.
    ///
    /// # Safety
    /// Uses legacy x86 PCI configuration I/O ports and must only run in ring 0.
    pub unsafe fn scan_network(&self, out: &mut [Option<PciDevice>]) -> usize {
        let mut count = 0usize;
        scan(|device| {
            if device.class == 0x02 && count < out.len() {
                out[count] = Some(device);
                count += 1;
            }
        });
        count
    }
}
