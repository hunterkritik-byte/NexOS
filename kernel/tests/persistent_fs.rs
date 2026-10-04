//! Host-side tests for the experimental fixed-slot persistent filesystem.
//!
//! Run with: cargo test -p nexkernel --test persistent_fs

#[path = "../src/fs/disk.rs"]
mod disk;
#[path = "../src/fs/persistent.rs"]
mod persistent;

use disk::{BlockDevice, DiskError, MemoryDisk};
use persistent::PersistentFs;

const MIN_BLOCKS: usize = 232;

#[test]
fn formats_and_mounts_an_empty_filesystem() {
    let fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    let mut fs = PersistentFs::mount(fs.disk).unwrap();

    let mut names: [&[u8]; 32] = [&[]; 32];
    let count = fs.list(&mut names);
    assert_eq!(count, 5);
    assert_eq!(\n        &names[..count],\n        &[&b"/"[..], &b"bin"[..], &b"etc"[..], &b"home"[..], &b"tmp"[..]],\n    );
}

#[test]
fn file_contents_survive_remount() {
    let mut fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    fs.create(b"hello.txt", b"hello, NexOS").unwrap();

    let mut fs = PersistentFs::mount(fs.disk).unwrap();
    let mut out = [0u8; 32];
    let count = fs.read(b"hello.txt", &mut out).unwrap();

    assert_eq!(&out[..count], b"hello, NexOS");
}

#[test]
fn reads_empty_partial_and_maximum_size_files() {
    let mut fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    fs.create(b"empty", b"").unwrap();
    fs.create(b"partial", b"sector boundary").unwrap();

    // Three bytes patterns across the full seven-sector fixed slot.
    let maximum = vec![0xA5; 7 * 512];
    fs.create(b"maximum", &maximum).unwrap();

    let mut empty = [0u8; 8];
    assert_eq!(fs.read(b"empty", &mut empty).unwrap(), 0);

    let mut partial = [0u8; 32];
    let count = fs.read(b"partial", &mut partial).unwrap();
    assert_eq!(&partial[..count], b"sector boundary");

    let mut actual = vec![0u8; maximum.len()];
    let count = fs.read(b"maximum", &mut actual).unwrap();
    assert_eq!(count, maximum.len());
    assert_eq!(actual, maximum);
}

#[test]
fn replacing_a_file_updates_its_length_and_contents() {
    let mut fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    fs.create(b"note", b"a much longer old value").unwrap();
    fs.write(b"note", b"new").unwrap();

    let mut out = [0xCC; 64];
    let count = fs.read(b"note", &mut out).unwrap();
    assert_eq!(&out[..count], b"new");
    assert_eq!(count, 3);
}

#[test]
fn rejects_duplicate_names_and_invalid_names() {
    let mut fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    fs.create(b"same", b"first").unwrap();

    assert_eq!(fs.create(b"same", b"second"), Err(DiskError::InvalidBuffer));
    assert_eq!(fs.mkdir(b"same"), Err(DiskError::InvalidBuffer));
    assert_eq!(fs.create(b"", b""), Err(DiskError::InvalidBuffer));
    assert_eq!(fs.create(b"bad\0name", b""), Err(DiskError::InvalidBuffer));
    assert_eq!(fs.create(&[b'x'; 65], b""), Err(DiskError::InvalidBuffer));
}

#[test]
fn rejects_file_operations_on_directories() {
    let mut fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    assert_eq!(fs.read(b"home", &mut [0u8; 8]), Err(DiskError::InvalidBuffer));
    assert_eq!(fs.write(b"home", b"nope"), Err(DiskError::InvalidBuffer));
}

#[test]
fn rejects_oversized_files_and_too_small_devices() {
    let mut fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    let too_large = vec![0u8; 7 * 512 + 1];
    assert_eq!(fs.create(b"large", &too_large), Err(DiskError::InvalidBuffer));

    assert!(matches!(
        PersistentFs::format(MemoryDisk::<231>::new()),
        Err(DiskError::OutOfBounds)
    ));
}

#[test]
fn mount_does_not_format_an_unrecognized_disk() {
    let mut disk = MemoryDisk::<MIN_BLOCKS>::new();
    let unknown_metadata = [0u8; 512];
    disk.write_block(0, &unknown_metadata).unwrap();

    assert!(matches!(
        PersistentFs::mount(disk),
        Err(DiskError::InvalidBuffer)
    ));
}

#[test]
fn mount_rejects_an_unknown_format_version() {
    let mut disk = MemoryDisk::<MIN_BLOCKS>::new();
    let mut header = [0u8; 512];
    header[0..4].copy_from_slice(&0x4e45_5846u32.to_le_bytes());
    header[4..8].copy_from_slice(&99u32.to_le_bytes());
    disk.write_block(0, &header).unwrap();

    assert!(matches!(
        PersistentFs::mount(disk),
        Err(DiskError::InvalidBuffer)
    ));
}

#[test]
fn memory_disk_rejects_out_of_range_blocks() {
    let mut disk = MemoryDisk::<4>::new();
    assert_eq!(disk.read_block(4, &mut [0u8; 512]), Err(DiskError::OutOfBounds));
    assert_eq!(disk.write_block(4, &[0u8; 512]), Err(DiskError::OutOfBounds));
}

#[test]
fn rejects_path_traversal_and_multi_component_names() {
    let mut fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();

    for name in [&b"."[..], &b".."[..], &b"../escape"[..], &b"dir/file"[..], &b"/absolute"[..]] {
        assert_eq!(
            fs.create(name, b"no"),
            Err(DiskError::InvalidBuffer),
            "accepted invalid file name: {:?}",
            name
        );
        assert_eq!(
            fs.mkdir(name),
            Err(DiskError::InvalidBuffer),
            "accepted invalid directory name: {:?}",
            name
        );
    }
}

#[test]
fn mount_rejects_directory_entries_with_file_metadata() {
    let fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    let mut disk = fs.disk;

    // Entry 1 is the default "bin" directory. Its size field begins at
    // header (8) + entry size (74) + fields before size (66).
    let mut sector = [0u8; 512];
    disk.read_block(0, &mut sector).unwrap();
    let size_offset = 8 + 74 + 66;
    sector[size_offset..size_offset + 4].copy_from_slice(&1u32.to_le_bytes());
    disk.write_block(0, &sector).unwrap();

    assert!(matches!(
        PersistentFs::mount(disk),
        Err(DiskError::InvalidBuffer)
    ));
}

#[test]
fn mount_rejects_noncanonical_name_padding() {
    let fs = PersistentFs::format(MemoryDisk::<MIN_BLOCKS>::new()).unwrap();
    let mut disk = fs.disk;

    // "bin" is entry 1. Bytes after its terminating NUL must all be zero.
    let mut sector = [0u8; 512];
    disk.read_block(0, &mut sector).unwrap();
    let name_offset = 8 + 74 + 2;
    sector[name_offset + 4] = b'x';
    disk.write_block(0, &sector).unwrap();

    assert!(matches!(
        PersistentFs::mount(disk),
        Err(DiskError::InvalidBuffer)
    ));
}
