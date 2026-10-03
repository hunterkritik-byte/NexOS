use x86_64::instructions::port::Port;

const CONFIG_ADDRESS: u16 = 0x0cf8;
const CONFIG_DATA: u16 = 0x0cfc;

#[derive(Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
}

impl PciDevice {
    pub const fn is_network(&self) -> bool {
        self.class == 0x02
    }
}

pub struct PciScanner;

impl PciScanner {
    pub const fn new() -> Self {
        Self
    }

    unsafe fn read_config(&self, bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
        let address = 0x8000_0000u32
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((function as u32) << 8)
            | ((offset as u32) & 0xfcu32);

        let mut address_port = Port::<u32>::new(CONFIG_ADDRESS);
        let mut data_port = Port::<u32>::new(CONFIG_DATA);
        address_port.write(address);
        data_port.read()
    }

    pub unsafe fn scan_network(&self, devices: &mut [Option<PciDevice>]) -> usize {
        let mut found = 0;

        for bus in 0..=255u16 {
            for slot in 0..32u8 {
                for function in 0..8u8 {
                    let id = self.read_config(bus as u8, slot, function, 0);
                    let vendor_id = (id & 0xffff) as u16;

                    if vendor_id == 0xffff {
                        if function == 0 {
                            break;
                        }
                        continue;
                    }

                    let device_id = (id >> 16) as u16;
                    let class_reg = self.read_config(bus as u8, slot, function, 0x08);
                    let class = (class_reg >> 24) as u8;
                    let subclass = (class_reg >> 16) as u8;

                    if class == 0x02 && found < devices.len() {
                        devices[found] = Some(PciDevice {
                            bus: bus as u8,
                            slot,
                            function,
                            vendor_id,
                            device_id,
                            class,
                            subclass,
                        });
                        found += 1;
                    }

                    let header = self.read_config(bus as u8, slot, function, 0x0c);
                    let multifunction = (header >> 16) as u8 & 0x80 != 0;
                    if function == 0 && !multifunction {
                        break;
                    }
                }
            }
        }

        found
    }
}
