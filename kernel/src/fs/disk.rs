#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskError { OutOfBounds, NotReady, ReadOnly, InvalidBuffer }

pub trait BlockDevice {
    const BLOCK_SIZE: usize = 512;
    fn block_count(&self) -> u64;
    fn read_block(&mut self, block:u64, dst:&mut [u8;Self::BLOCK_SIZE])->Result<(),DiskError>;
    fn write_block(&mut self, block:u64, src:&[u8;Self::BLOCK_SIZE])->Result<(),DiskError>;
}

/// ATA PIO primary-master disk. This is intentionally synchronous; IRQ/DMA can
/// be added later without changing the BlockDevice interface.
pub struct AtaPioDisk { sectors:u64 }

impl AtaPioDisk {
    pub const unsafe fn new(sectors:u64)->Self { Self{sectors} }
    unsafe fn wait(&mut self)->Result<(),DiskError> {
        use x86_64::instructions::port::Port;
        let mut status=Port::<u8>::new(0x1f7);
        for _ in 0..1_000_000 {
            let s=status.read();
            if s & 0x01 != 0 { return Err(DiskError::NotReady); }
            if s & 0x80 == 0 && s & 0x08 != 0 { return Ok(()); }
        }
        Err(DiskError::NotReady)
    }
}
impl BlockDevice for AtaPioDisk {
    fn block_count(&self)->u64 { self.sectors }
    fn read_block(&mut self, block:u64, dst:&mut [u8;512])->Result<(),DiskError> {
        if block>=self.sectors{return Err(DiskError::OutOfBounds)}
        unsafe {
            use x86_64::instructions::port::Port;
            let mut count=Port::<u8>::new(0x1f2); let mut lba0=Port::<u8>::new(0x1f3);
            let mut lba1=Port::<u8>::new(0x1f4); let mut lba2=Port::<u8>::new(0x1f5);
            let mut drive=Port::<u8>::new(0x1f6); let mut cmd=Port::<u8>::new(0x1f7);
            count.write(1); lba0.write(block as u8); lba1.write((block>>8) as u8);
            lba2.write((block>>16) as u8); drive.write(0xe0|((block>>24)&0x0f) as u8);
            cmd.write(0x20); self.wait()?;
            let mut data=Port::<u16>::new(0x1f0);
            for i in 0..256 { dst[i*2..i*2+2].copy_from_slice(&data.read().to_le_bytes()); }
        }
        Ok(())
    }
    fn write_block(&mut self, block:u64, src:&[u8;512])->Result<(),DiskError> {
        if block>=self.sectors{return Err(DiskError::OutOfBounds)}
        unsafe {
            use x86_64::instructions::port::Port;
            let mut count=Port::<u8>::new(0x1f2); let mut lba0=Port::<u8>::new(0x1f3);
            let mut lba1=Port::<u8>::new(0x1f4); let mut lba2=Port::<u8>::new(0x1f5);
            let mut drive=Port::<u8>::new(0x1f6); let mut cmd=Port::<u8>::new(0x1f7);
            count.write(1); lba0.write(block as u8); lba1.write((block>>8) as u8);
            lba2.write((block>>16) as u8); drive.write(0xe0|((block>>24)&0x0f) as u8);
            cmd.write(0x30); self.wait()?;
            let mut data=Port::<u16>::new(0x1f0);
            for i in 0..256 { data.write(u16::from_le_bytes([src[i*2],src[i*2+1]])); }
            cmd.write(0xe7);
        }
        Ok(())
    }
}

pub struct MemoryDisk<const BLOCKS:usize>{ blocks:[[u8;512];BLOCKS] }
impl<const BLOCKS:usize> MemoryDisk<BLOCKS>{ pub const fn new()->Self{Self{blocks:[[0;512];BLOCKS]}} }
impl<const BLOCKS:usize> BlockDevice for MemoryDisk<BLOCKS>{
    fn block_count(&self)->u64{BLOCKS as u64}
    fn read_block(&mut self,b:u64,d:&mut [u8;512])->Result<(),DiskError>{if b>=BLOCKS as u64{return Err(DiskError::OutOfBounds)}*d=self.blocks[b as usize];Ok(())}
    fn write_block(&mut self,b:u64,s:&[u8;512])->Result<(),DiskError>{if b>=BLOCKS as u64{return Err(DiskError::OutOfBounds)}self.blocks[b as usize]=*s;Ok(())}
}
