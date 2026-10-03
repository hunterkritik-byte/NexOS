pub mod pci;
pub mod virtio_net;

pub fn is_network_controller(class:u8, subclass:u8)->bool {
    class == 0x02 && (subclass == 0x00 || subclass == 0x80)
}
