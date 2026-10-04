# Desktop input routing

NexOS now has a small, host-testable pointer interaction layer in
`kernel/src/graphics/input.rs`. It consumes the existing `InputEvent` type
and applies pointer actions to the window manager.

## Supported behavior

- Mouse movement updates a non-negative desktop cursor position.
- Primary button (button 0) press focuses and raises the front-most window
  under the cursor.
- Pressing the title bar (up to 20 pixels tall) begins dragging that window.
- Releasing the primary button ends a drag.
- Non-primary buttons and keyboard events are ignored by this pointer router.

## Current limitations

This is an interaction primitive, not a complete desktop session. No mouse
driver currently feeds hardware events into this router, and the kernel boot
path does not yet create a desktop session or connect the router to the live
framebuffer/compositor. Cursor drawing, title text, window controls, app
surfaces, USB HID support, and physical-hardware validation remain future work.

The host-side regression tests are in `kernel/tests/desktop_input.rs`. Run
them from the repository root with:

```sh
cargo test -p nexkernel --test desktop_input
```
