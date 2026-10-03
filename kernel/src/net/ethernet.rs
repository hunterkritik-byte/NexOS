#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct EthernetHeader {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ethertype: u16,
}

pub const ETHERTYPE_ARP: u16 = 0x0806;
pub const ETHERTYPE_IPV4: u16 = 0x0800;

pub fn ethertype(bytes: &[u8]) -> Option<u16> {
    if bytes.len() < 14 { return None; }
    Some(u16::from_be_bytes([bytes[12], bytes[13]]))
}
