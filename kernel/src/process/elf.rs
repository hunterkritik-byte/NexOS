#![allow(dead_code)]

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[derive(Clone, Copy)]
pub struct Elf64Header {
    pub entry: u64,
    pub phoff: u64,
    pub phentsize: u16,
    pub phnum: u16,
}

pub fn parse_elf64(image: &[u8]) -> Result<Elf64Header, &'static str> {
    if image.len() < 64 || image[0..4] != ELF_MAGIC {
        return Err("not an ELF image");
    }

    if image[4] != 2 || image[5] != 1 {
        return Err("ELF image is not 64-bit little-endian");
    }

    let entry = u64::from_le_bytes(image[24..32].try_into().unwrap());
    let phoff = u64::from_le_bytes(image[32..40].try_into().unwrap());
    let phentsize = u16::from_le_bytes(image[54..56].try_into().unwrap());
    let phnum = u16::from_le_bytes(image[56..58].try_into().unwrap());

    if phentsize < 56 {
        return Err("invalid ELF program-header size");
    }

    Ok(Elf64Header { entry, phoff, phentsize, phnum })
}
