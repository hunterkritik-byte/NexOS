pub mod pci;
pub mod pci_nic;
pub mod virtio_net;

pub fn is_network_controller(class:u8, _subclass:u8)->bool { class==0x02 }
