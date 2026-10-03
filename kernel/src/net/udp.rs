#[repr(C, packed)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

pub fn parse(packet: &[u8]) -> Option<(u16, u16, &[u8])> {
    if packet.len() < 8 { return None; }
    let len = u16::from_be_bytes([packet[4], packet[5]]) as usize;
    if len < 8 || len > packet.len() { return None; }
    Some((
        u16::from_be_bytes([packet[0], packet[1]]),
        u16::from_be_bytes([packet[2], packet[3]]),
        &packet[8..len],
    ))
}
