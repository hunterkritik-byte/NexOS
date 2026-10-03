use core::net::Ipv4Addr;

#[derive(Clone, Copy)]
pub struct ArpEntry {
    pub ip: Ipv4Addr,
    pub mac: [u8; 6],
}

pub struct ArpTable {
    entries: [Option<ArpEntry>; 32],
}

impl ArpTable {
    pub const fn new() -> Self { Self { entries: [None; 32] } }

    pub fn insert(&mut self, entry: ArpEntry) {
        if let Some(slot) = self.entries.iter_mut().find(|e| e.map_or(false, |x| x.ip == entry.ip)) {
            *slot = Some(entry); return;
        }
        if let Some(slot) = self.entries.iter_mut().find(|e| e.is_none()) { *slot = Some(entry); }
    }

    pub fn lookup(&self, ip: Ipv4Addr) -> Option<[u8; 6]> {
        self.entries.iter().flatten().find(|e| e.ip == ip).map(|e| e.mac)
    }
}
