use super::elf::parse_elf64;

#[derive(Clone, Copy)]
pub struct LoadedProgram {
    pub entry: u64,
    pub stack_top: u64,
}

/// Validates an ELF image and returns its entry point.
/// Segment copying/mapping is deliberately kept separate so it can enforce
/// USER_ACCESSIBLE + NX/W^X policy with the physical frame allocator.
pub fn inspect(image: &[u8]) -> Result<LoadedProgram, &'static str> {
    let header = parse_elf64(image)?;
    if header.entry < 0x400000 || header.entry >= 0x0000_8000_0000_0000 {
        return Err("ELF entry is outside the user address range");
    }
    Ok(LoadedProgram {
        entry: header.entry,
        stack_top: 0x0000_0080_0000,
    })
}
