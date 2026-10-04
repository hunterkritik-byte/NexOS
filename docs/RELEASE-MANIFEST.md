# NexOS Release Manifest

Every release should record:

- NexOS version
- Git commit
- Debian base release
- Architecture: x86_64 / amd64
- Kernel version
- ISO filename
- ISO byte size
- SHA-256
- SHA-512
- Build timestamp
- Build environment
- Package manifest
- SBOM
- QEMU smoke-test result
- Known hardware limitations

This prevents a small metadata artifact from being mistaken for a complete NexOS installation image.
