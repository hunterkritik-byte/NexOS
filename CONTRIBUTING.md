# Contributing to NexOS

NexOS is a from-scratch operating-system project.

## Before contributing

Read README.md, docs/ARCHITECTURE.md, docs/ROADMAP.md, and SECURITY.md.

## Rules

- Do not describe placeholders as implemented features.
- Keep kernel and userspace boundaries explicit.
- Document every new unsafe block.
- Add regression tests for fixed bugs.
- Prefer small, reviewable changes.
- Keep architecture-specific code under kernel/src.
- Do not commit generated ISO or disk images.

## Testing

    cargo fmt --all -- --check
    cargo check --workspace
    cargo build --release

For boot changes, test both UEFI and BIOS in QEMU.
