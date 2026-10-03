use core::net::Ipv4Addr;

pub struct DnsConfig {
    pub server: Ipv4Addr,
}

pub const DNS_PORT: u16 = 53;
