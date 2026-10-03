use core::net::Ipv4Addr;

#[derive(Clone, Copy)]
pub enum SocketKind { Udp, Tcp }

#[derive(Clone, Copy)]
pub struct Socket {
    pub kind: SocketKind,
    pub local: Option<(Ipv4Addr, u16)>,
    pub remote: Option<(Ipv4Addr, u16)>,
}
