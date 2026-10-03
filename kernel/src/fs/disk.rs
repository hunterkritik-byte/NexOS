#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskError {
    OutOfBounds,
    NotReady,
    ReadOnly,
}

/// Block-device interface used by the VFS/filesystems.
/// Implementations must provide persistent storage.
pub trait BlockDevice {
    const BLOCK_SIZE: usize = 512;

    fn block_count(&self) -> u64;
    fn read_block(&mut self, block: u64, dst: &mut [u8; Self::BLOCK_SIZE]) -> Result<(), DiskError>;
    fn write_block(&mut self, block: u64, src: &[u8; Self::BLOCK_SIZE]) -> Result<(), DiskError>;
}

/// In-memory device is deliberately only a development/test backend.
/// It is not persistent across reboot.
pub struct MemoryDisk<const BLOCKS: usize> {
    blocks: [[u8; BlockDevice::BLOCK_SIZE]; BLOCKS],
}

impl<const BLOCKS: usize> MemoryDisk<BLOCKS> {
    pub const fn new() -> Self {
        Self { blocks: [[0; BlockDevice::BLOCK_SIZE]; BLOCKS] }
    }
}

impl<const BLOCKS: usize> BlockDevice for MemoryDisk<BLOCKS> {
    fn block_count(&self) -> u64 { BLOCKS as u64 }

    fn read_block(&mut self, block: u64, dst: &mut [u8; Self::BLOCK_SIZE]) -> Result<(), DiskError> {
        let index = block as usize;
        if index >= BLOCKS { return Err(DiskError::OutOfBounds); }
        *dst = self.blocks[index];
        Ok(())
    }

    fn write_block(&mut self, block: u64, src: &[u8; Self::BLOCK_SIZE]) -> Result<(), DiskError> {
        let index = block as usize;
        if index >= BLOCKS { return Err(DiskError::OutOfBounds); }
        self.blocks[index] = *src;
        Ok(())
    }
}
