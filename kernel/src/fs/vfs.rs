use super::disk::{BlockDevice, DiskError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    AlreadyExists,
    NotDirectory,
    IsDirectory,
    InvalidPath,
    NoSpace,
    Io(DiskError),
}

pub struct File {
    pub size: usize,
    pub data: [u8; 4096],
}

impl File {
    pub const fn empty() -> Self {
        Self { size: 0, data: [0; 4096] }
    }
}

pub trait FileSystem {
    fn mkdir(&mut self, path: &str) -> Result<(), FsError>;
    fn create(&mut self, path: &str) -> Result<(), FsError>;
    fn write(&mut self, path: &str, data: &[u8]) -> Result<usize, FsError>;
    fn read(&self, path: &str, out: &mut [u8]) -> Result<usize, FsError>;
}

#[derive(Clone, Copy)]
struct Entry {
    used: bool,
    directory: bool,
    path: [u8; 128],
    path_len: usize,
    file: File,
}

impl Entry {
    const fn empty() -> Self {
        Self {
            used: false,
            directory: false,
            path: [0; 128],
            path_len: 0,
            file: File::empty(),
        }
    }
}

pub struct Vfs<const ENTRIES: usize> {
    entries: [Entry; ENTRIES],
}

impl<const ENTRIES: usize> Vfs<ENTRIES> {
    pub const fn new() -> Self {
        Self { entries: [Entry::empty(); ENTRIES] }
    }

    fn find(&self, path: &str) -> Option<usize> {
        self.entries.iter().position(|e| {
            e.used && e.path_len == path.len() && e.path[..e.path_len] == *path.as_bytes()
        })
    }

    fn add(&mut self, path: &str, directory: bool) -> Result<(), FsError> {
        if path.is_empty() || !path.starts_with('/') || path.len() > 128 {
            return Err(FsError::InvalidPath);
        }
        if self.find(path).is_some() {
            return Err(FsError::AlreadyExists);
        }
        let index = self.entries.iter().position(|e| !e.used).ok_or(FsError::NoSpace)?;
        let entry = &mut self.entries[index];
        entry.used = true;
        entry.directory = directory;
        entry.path_len = path.len();
        entry.path[..path.len()].copy_from_slice(path.as_bytes());
        Ok(())
    }

    pub fn format_and_mount(&mut self) -> Result<(), FsError> {
        self.add("/", true)?;
        for path in ["/bin", "/etc", "/home", "/tmp", "/var", "/dev", "/proc", "/sbin"] {
            self.add(path, true)?;
        }
        self.add("/etc/motd", false)?;
        self.write("/etc/motd", b"Welcome to NexOS.\n").map(|_| ())
    }

    pub fn list_root(&self, out: &mut [&str; 16]) -> usize {
        // Static paths are exposed by the filesystem namespace.
        // A future directory iterator will replace this fixed-size API.
        let mut n = 0;
        for entry in &self.entries {
            if entry.used && entry.path_len > 1 && entry.path[..entry.path_len].iter().filter(|&&b| b == b'/').count() == 1 {
                // The caller currently uses this only as a count/diagnostic.
                if n < out.len() { out[n] = ""; n += 1; }
            }
        }
        n
    }
}

impl<const ENTRIES: usize> FileSystem for Vfs<ENTRIES> {
    fn mkdir(&mut self, path: &str) -> Result<(), FsError> {
        self.add(path, true)
    }

    fn create(&mut self, path: &str) -> Result<(), FsError> {
        self.add(path, false)
    }

    fn write(&mut self, path: &str, data: &[u8]) -> Result<usize, FsError> {
        let index = self.find(path).ok_or(FsError::NotFound)?;
        let entry = &mut self.entries[index];
        if entry.directory { return Err(FsError::IsDirectory); }
        if data.len() > entry.file.data.len() { return Err(FsError::NoSpace); }
        entry.file.data[..data.len()].copy_from_slice(data);
        entry.file.size = data.len();
        Ok(data.len())
    }

    fn read(&self, path: &str, out: &mut [u8]) -> Result<usize, FsError> {
        let index = self.find(path).ok_or(FsError::NotFound)?;
        let entry = &self.entries[index];
        if entry.directory { return Err(FsError::IsDirectory); }
        let len = core::cmp::min(entry.file.size, out.len());
        out[..len].copy_from_slice(&entry.file.data[..len]);
        Ok(len)
    }
}
