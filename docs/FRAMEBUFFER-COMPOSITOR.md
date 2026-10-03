# Framebuffer compositor adapter

The kernel's framebuffer wrapper now implements the compositor's
`PixelCanvas` interface. The adapter reports the framebuffer's visible
width and height and delegates pixel writes to the existing bounds-checked
RGB/BGR pixel writer.

The graphics module is also declared in the kernel module tree, with the
window manager re-exported at the path currently used by the graphics
components. This makes the graphics foundations part of the kernel build
rather than isolated source files.

## What this does not do yet

This change does not acquire the bootloader framebuffer, start a desktop at
boot, or call the compositor's `render` function in the running kernel.
Framebuffer ownership and boot-time rendering still need to be wired into
the boot path. Unsupported framebuffer pixel formats remain ignored by the
existing pixel writer.

A compile-time integration test is in `kernel/tests/framebuffer_canvas.rs`.
Run it with:

```sh
cargo test -p nexkernel --test framebuffer_canvas
```
