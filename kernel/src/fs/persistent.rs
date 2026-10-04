use super::disk::{BlockDevice, DiskError};

const MAGIC: u32 = 0x4e45_5846;
const VERSION: u32 = 1;
const MAX_FILES: usize = 32;
const NAME_BYTES: usize = 64;
const ENTRY_BYTES: usize = 74;
const METADATA_BYTES: usize = 8 + MAX_FILES * ENTRY_BYTES;
const METADATA_BLOCKS: usize = (METADATA_BYTES + 511) / 512;
const DATA_START: u64 = 8;
const DATA_BLOCKS_PER_ENTRY: u64 = 7;
const MAX_FILE_BYTES: usize = DATA_BLOCKS_PER_ENTRY as usize * 512;

#[derive(Clone, Copy)]
struct Dirent {
    used: u8,
    directory: u8,
    name: [u8; NAME_BYTES],
    size: u32,
    first: u32,
}

impl Dirent {
    const fn empty() -> Self {
        Self {
            used: 0,
            directory: 0,
            name: [0; NAME_BYTES],
            size: 0,
            first: 0,
        }
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(NAME_BYTES);
        &self.name[..len]
    }
}

/// A deliberately small fixed-slot filesystem for early OS bring-up.
///
/// Metadata occupies sectors 0..METADATA_BLOCKS. File data is allocated in
/// non-overlapping fixed slots after sector 8. This is not FAT/ext2 and does
/// not provide journaling, permissions, nested directories, or crash recovery.
pub struct PersistentFs<D: BlockDevice> {
    pub disk: D,
    entries: [Dirent; MAX_FILES],
}

impl<D: BlockDevice> PersistentFs<D> {
    pub fn mount(mut disk: D) -> Result<Self, DiskError> {
        if disk.block_count() < DATA_START + MAX_FILES as u64 * DATA_BLOCKS_PER_ENTRY {
            return Err(DiskError::OutOfBounds);
        }

        let mut fs = Self {
            disk,
            entries: [Dirent::empty(); MAX_FILES],
        };
        let mut metadata = [0u8; METADATA_BLOCKS * 512];
        fs.read_metadata(&mut metadata)?;

        let magic = u32::from_le_bytes(metadata[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(metadata[4..8].try_into().unwrap());
        // Mount must never overwrite an unknown disk: it may contain another
        // filesystem or recoverable data. Formatting is an explicit operation.
        if magic != MAGIC || version != VERSION {
            return Err(DiskError::InvalidBuffer);
        }
        fs.decode_metadata(&metadata)?;
        Ok(fs)
    }

    /// Explicitly initialize a new NexOS filesystem on the supplied device.
    ///
    /// WARNING: this overwrites the filesystem metadata sectors. Call only for
    /// a disposable/new disk after the caller has selected the target device.
    pub fn format(mut disk: D) -> Result<Self, DiskError> {
        if disk.block_count() < DATA_START + MAX_FILES as u64 * DATA_BLOCKS_PER_ENTRY {
            return Err(DiskError::OutOfBounds);
        }
        let mut fs = Self {
            disk,
            entries: [Dirent::empty(); MAX_FILES],
        };
        fs.initialize()?;
        Ok(fs)
    }

    fn read_metadata(&mut self, metadata: &mut [u8; METADATA_BLOCKS * 512]) -> Result<(), DiskError> {
        for block in 0..METADATA_BLOCKS {
            let mut sector = [0u8; 512];
            self.disk.read_block(block as u64, &mut sector)?;
            let start = block * 512;
            metadata[start..start + 512].copy_from_slice(&sector);
        }
        Ok(())
    }

    fn decode_metadata(&mut self, metadata: &[u8; METADATA_BLOCKS * 512]) -> Result<(), DiskError> {
        let mut offset = 8;
        for index in 0..MAX_FILES {
            let end = offset + ENTRY_BYTES;
            let raw = &metadata[offset..end];
            let mut entry = Dirent::empty();
            entry.used = raw[0];
            entry.directory = raw[1];
            entry.name.copy_from_slice(&raw[2..2 + NAME_BYTES]);
            entry.size = u32::from_le_bytes(raw[66..70].try_into().unwrap());
            entry.first = u32::from_le_bytes(raw[70..74].try_into().unwrap());

            if entry.used > 1 || entry.directory > 1 {
                return Err(DiskError::InvalidBuffer);
            }
            if entry.used == 1 {
                let name = entry.name();
                if name.is_empty() || entry.size as usize > MAX_FILE_BYTES {
                    return Err(DiskError::InvalidBuffer);
                }

                // Names are single components. The only slash-containing name
                // permitted by this prototype is the root directory "/".
                if name == b"/" {
                    if entry.directory == 0 || index != 0 {
                        return Err(DiskError::InvalidBuffer);
                    }
                } else if name == b"." || name == b".." || name.contains(&b'/') {
                    return Err(DiskError::InvalidBuffer);
                }

                // Require canonical zero padding after the first NUL. Without
                // this, multiple byte representations could name the same file.
                if let Some(nul) = entry.name.iter().position(|&b| b == 0) {
                    if entry.name[nul + 1..].iter().any(|&b| b != 0) {
                        return Err(DiskError::InvalidBuffer);
                    }
                }

                if entry.directory == 1 {
                    if entry.size != 0 || entry.first != 0 {
                        return Err(DiskError::InvalidBuffer);
                    }
                } else {
                    let first = entry.first as u64;
                    let limit = first.checked_add(DATA_BLOCKS_PER_ENTRY).ok_or(DiskError::OutOfBounds)?;
                    if first < DATA_START || limit > self.disk.block_count() {
                        return Err(DiskError::OutOfBounds);
                    }
                    let expected_first = DATA_START + index as u64 * DATA_BLOCKS_PER_ENTRY;
                    if first != expected_first {
                        return Err(DiskError::InvalidBuffer);
                    }
                }
                for previous in &self.entries[..index] {
                    if previous.used == 1 && previous.name() == name {
                        return Err(DiskError::InvalidBuffer);
                    }
                }
            }
            self.entries[index] = entry;
            offset = end;
        }
        Ok(())
    }

    fn initialize(&mut self) -> Result<(), DiskError> {
        self.entries = [Dirent::empty(); MAX_FILES];
        self.insert_entry(b"/", true, 0)?;
        for name in [b"bin".as_slice(), b"etc", b"home", b"tmp"] {
            self.insert_entry(name, true, 0)?;
        }
        self.sync()
    }

    fn insert_entry(&mut self, name: &[u8], directory: bool, size: usize) -> Result<usize, DiskError> {
        if name.is_empty() || name.len() > NAME_BYTES || name.contains(&0) {
            return Err(DiskError::InvalidBuffer);
        }
        if name == b"/" {
            if !directory {
                return Err(DiskError::InvalidBuffer);
            }
        } else if name == b"." || name == b".." || name.contains(&b'/') {
            return Err(DiskError::InvalidBuffer);
        }
        if self.find(name).is_some() {
            return Err(DiskError::InvalidBuffer);
        }
        let index = self.entries.iter().position(|e| e.used == 0).ok_or(DiskError::OutOfBounds)?;
        let mut entry = Dirent::empty();
        entry.used = 1;
        entry.directory = u8::from(directory);
        entry.name[..name.len()].copy_from_slice(name);
        entry.size = size as u32;
        if !directory {
            entry.first = (DATA_START + index as u64 * DATA_BLOCKS_PER_ENTRY) as u32;
        }
        self.entries[index] = entry;
        Ok(index)
    }

    fn find(&self, name: &[u8]) -> Option<usize> {
        self.entries.iter().position(|e| e.used == 1 && e.name() == name)
    }

    fn sync(&mut self) -> Result<(), DiskError> {
        let mut metadata = [0u8; METADATA_BLOCKS * 512];
        metadata[0..4].copy_from_slice(&MAGIC.to_le_bytes());
        metadata[4..8].copy_from_slice(&VERSION.to_le_bytes());
        let mut offset = 8;
        for entry in &self.entries {
            metadata[offset] = entry.used;
            metadata[offset + 1] = entry.directory;
            metadata[offset + 2..offset + 2 + NAME_BYTES].copy_from_slice(&entry.name);
            metadata[offset + 66..offset + 70].copy_from_slice(&entry.size.to_le_bytes());
            metadata[offset + 70..offset + 74].copy_from_slice(&entry.first.to_le_bytes());
            offset += ENTRY_BYTES;
        }
        for block in 0..METADATA_BLOCKS {
            let mut sector = [0u8; 512];
            let start = block * 512;
            sector.copy_from_slice(&metadata[start..start + 512]);
            self.disk.write_block(block as u64, &sector)?;
        }
        Ok(())
    }

    /// Create a directory with a single-component name (not a path).
    pub fn mkdir(&mut self, name: &[u8]) -> Result<(), DiskError> {
        self.insert_entry(name, true, 0)?;
        self.sync()
    }

    /// Create a file and write its initial contents.
    pub fn create(&mut self, name: &[u8], data: &[u8]) -> Result<(), DiskError> {
        if data.len() > MAX_FILE_BYTES {
            return Err(DiskError::InvalidBuffer);
        }
        let index = self.insert_entry(name, false, data.len())?;
        if let Err(error) = self.write_entry_data(index, data) {
            self.entries[index] = Dirent::empty();
            return Err(error);
        }
        self.sync()
    }

    /// Replace the contents of an existing file.
    pub fn write(&mut self, name: &[u8], data: &[u8]) -> Result<(), DiskError> {
        if data.len() > MAX_FILE_BYTES {
            return Err(DiskError::InvalidBuffer);
        }
        let index = self.find(name).ok_or(DiskError::OutOfBounds)?;
        if self.entries[index].directory != 0 {
            return Err(DiskError::InvalidBuffer);
        }
        self.write_entry_data(index, data)?;
        self.entries[index].size = data.len() as u32;
        self.sync()
    }

    fn write_entry_data(&mut self, index: usize, data: &[u8]) -> Result<(), DiskError> {
        let first = self.entries[index].first as u64;
        for slot in 0..DATA_BLOCKS_PER_ENTRY as usize {
            let mut sector = [0u8; 512];
            let start = slot * 512;
            if start < data.len() {
                let end = (start + 512).min(data.len());
                sector[..end - start].copy_from_slice(&data[start..end]);
            }
            self.disk.write_block(first + slot as u64, &sector)?;
        }
        Ok(())
    }

    /// Read file contents into caller-owned storage; returns bytes copied.
    pub fn read(&mut self, name: &[u8], out: &mut [u8]) -> Result<usize, DiskError> {
        let index = self.find(name).ok_or(DiskError::OutOfBounds)?;
        let entry = self.entries[index];
        if entry.directory != 0 {
            return Err(DiskError::InvalidBuffer);
        }
        let count = core::cmp::min(entry.size as usize, out.len());
        let mut copied = 0;
        for slot in 0..DATA_BLOCKS_PER_ENTRY as usize {
            if copied >= count {
                break;
            }
            let mut sector = [0u8; 512];
            self.disk.read_block(entry.first as u64 + slot as u64, &mut sector)?;
            let take = core::cmp::min(512, count - copied);
            out[copied..copied + take].copy_from_slice(&sector[..take]);
            copied += take;
        }
        Ok(copied)
    }

    pub fn list<'a>(&'a self, out: &mut [&'a [u8]; MAX_FILES]) -> usize {
        let mut count = 0;
        for entry in &self.entries {
            if entry.used == 1 && count < out.len() {
                out[count] = entry.name();
                count += 1;
            }
        }
        count
    }
}
