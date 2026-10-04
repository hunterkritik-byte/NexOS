#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsError { NotFound, AlreadyExists, InvalidPath, NotDirectory, NoSpace, TooLarge }

pub struct Vfs<const N: usize> {
    entries: [Option<Entry>; N],
}

#[derive(Clone, Copy)]
struct Entry {
    path: [u8; 128],
    len: usize,
    data: [u8; 512],
    data_len: usize,
    directory: bool,
}

impl<const N: usize> Vfs<N> {
    pub const fn new() -> Self { Self { entries: [None; N] } }

    pub fn format_and_mount(&mut self) -> Result<(), FsError> {
        self.mkdir("/")?;
        for p in ["/bin","/dev","/etc","/home","/proc","/sbin","/tmp","/var"] {
            let _ = self.mkdir(p);
        }
        Ok(())
    }

    fn find(&self, path: &str) -> Option<usize> {
        self.entries.iter().position(|e| e.as_ref().map(|x| x.matches(path)).unwrap_or(false))
    }

    pub fn create(&mut self, path: &str) -> Result<(), FsError> {
        if path.is_empty() || !path.starts_with('/') { return Err(FsError::InvalidPath); }
        if self.find(path).is_some() { return Err(FsError::AlreadyExists); }
        let i = self.entries.iter().position(Option::is_none).ok_or(FsError::NoSpace)?;
        self.entries[i] = Some(Entry::new(path, false)?);
        Ok(())
    }

    pub fn mkdir(&mut self, path: &str) -> Result<(), FsError> {
        if self.find(path).is_some() { return Err(FsError::AlreadyExists); }
        let i = self.entries.iter().position(Option::is_none).ok_or(FsError::NoSpace)?;
        self.entries[i] = Some(Entry::new(path, true)?);
        Ok(())
    }

    pub fn write(&mut self, path: &str, data: &[u8]) -> Result<usize, FsError> {
        let i = self.find(path).ok_or(FsError::NotFound)?;
        let e = self.entries[i].as_mut().ok_or(FsError::NotFound)?;
        if e.directory { return Err(FsError::NotDirectory); }
        if data.len() > e.data.len() { return Err(FsError::TooLarge); }
        e.data[..data.len()].copy_from_slice(data);
        e.data_len = data.len();
        Ok(data.len())
    }

    pub fn read(&self, path: &str, out: &mut [u8]) -> Result<usize, FsError> {
        let i = self.find(path).ok_or(FsError::NotFound)?;
        let e = self.entries[i].as_ref().ok_or(FsError::NotFound)?;
        if e.directory { return Err(FsError::NotDirectory); }
        let n = core::cmp::min(out.len(), e.data_len);
        out[..n].copy_from_slice(&e.data[..n]);
        Ok(n)
    }
}

impl Entry {
    fn new(path: &str, directory: bool) -> Result<Self, FsError> {
        if path.len() > 128 { return Err(FsError::InvalidPath); }
        let mut p=[0u8;128]; p[..path.len()].copy_from_slice(path.as_bytes());
        Ok(Self { path:p, len:path.len(), data:[0;512], data_len:0, directory })
    }
    fn matches(&self, path: &str) -> bool { self.len == path.len() && self.path[..self.len] == *path.as_bytes() }
}

pub struct FileSystem;
impl FileSystem {
    pub fn create<const N: usize>(v: &mut Vfs<N>, p: &str) -> Result<(), FsError> { v.create(p) }
    pub fn mkdir<const N: usize>(v: &mut Vfs<N>, p: &str) -> Result<(), FsError> { v.mkdir(p) }
    pub fn write<const N: usize>(v: &mut Vfs<N>, p: &str, d: &[u8]) -> Result<usize, FsError> { v.write(p,d) }
    pub fn read<const N: usize>(v: &Vfs<N>, p: &str, o: &mut [u8]) -> Result<usize, FsError> { v.read(p,o) }
}
