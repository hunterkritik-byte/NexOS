# Persistent filesystem prototype

NexOS's `PersistentFs` is an early fixed-slot filesystem for development and disposable test disks. It is not a general-purpose filesystem and is not currently mounted by the normal boot path.

## Safety rules

- `PersistentFs::mount(device)` reads metadata and returns an error if the magic or version is unknown. It must not format an unrecognized disk.
- `PersistentFs::format(device)` is destructive to the filesystem metadata area. Call it only after the caller has explicitly selected a new or disposable device.
- Never point this prototype at a host disk or a disk containing user data.
- A successful write is not a guarantee of crash consistency. There is no journal, redundant superblock, checksum, or recovery protocol.
- The ATA PIO implementation is still a prototype. Do not treat this filesystem as safe for real hardware data.

## Layout (version 1)

| Region | Sectors | Purpose |
| --- | ---: | --- |
| Metadata | 0–4 | Magic/version and 32 directory entries |
| Reserved gap | 5–7 | Reserved for future metadata |
| Data slots | 8 onward | One fixed seven-sector slot per entry index |

Each directory entry is 74 bytes: used flag (1), directory flag (1), name (64), file size (4), and first data sector (4), all integer fields little-endian. The metadata header is eight bytes. The metadata area is five 512-byte sectors.

A file can contain at most 3,584 bytes. Its slot is determined by its directory-entry index, so this design has no free-space allocator and no fragmentation handling. Directory names are single components; nested paths, deletion, rename, permissions, timestamps, and symbolic links are not supported.

The minimum device size is 232 sectors: eight reserved sectors plus 32 slots of seven sectors each.

## API sketch

```rust
// Explicitly initialize a disposable/new test disk (destructive).
let mut fs = PersistentFs::format(MemoryDisk::<232>::new())?;

// Mount a previously formatted disk. Unknown metadata returns an error.
let mut fs = PersistentFs::mount(disk)?;
fs.create(b"hello.txt", b"hello, NexOS")?;

let mut buffer = [0u8; 64];
let count = fs.read(b"hello.txt", &mut buffer)?;
assert_eq!(&buffer[..count], b"hello, NexOS");
```

The snippet documents intended usage; it is not a claim that the current repository has compiled or executed these tests.

## Validation checklist

Before integration into boot or shell commands, add/run tests for:

- explicit formatting and remount persistence on the same in-memory block device;
- rejection of unformatted, unknown-version, truncated, duplicate-name, and out-of-range metadata;
- empty, partial-sector, and maximum-size file round trips;
- duplicate file/directory names and operations on directories as files;
- disk read/write failures and behavior after interrupted metadata updates.

Mounting and formatting should remain separate operations. Any future auto-detection must never silently overwrite unknown data.
