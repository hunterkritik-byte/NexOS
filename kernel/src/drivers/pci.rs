#[derive(Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub bars: [u32; 6],
}

unsafe fn config_address(bus:u8, device:u8, function:u8, offset:u8)->u32 {
    0x8000_0000
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xfc)
}

pub unsafe fn config_read32(bus:u8, device:u8, function:u8, offset:u8)->u32 {
    use x86_64::instructions::port::Port;
    let mut address = Port::<u32>::new(0xcf8);
    let mut data = Port::<u32>::new(0xcfc);
    address.write(config_address(bus,device,function,offset));
    data.read()
}

pub unsafe fn scan<F: FnMut(PciDevice)>(mut callback:F) {
    for bus in 0..=255 {
        for device in 0..32 {
            let vendor = config_read32(bus,device,0,0) as u16;
            if vendor == 0xffff { continue; }
            for function in 0..8 {
                let id = config_read32(bus,device,function,0);
                if id as u16 == 0xffff { continue; }
                let class_reg = config_read32(bus,device,function,8);
                let mut bars=[0;6];
                for i in 0..6 { bars[i]=config_read32(bus,device,function,0x10+i*4); }
                callback(PciDevice {
                    bus,device,function,
                    vendor_id:id as u16,
                    device_id:(id>>16) as u16,
                    class:(class_reg>>24) as u8,
                    subclass:(class_reg>>16) as u8,
                    prog_if:(class_reg>>8) as u8,
                    bars,
                });
            }
        }
    }
}
