# Graphics text console

The graphics console renders readable text onto any implementation of the
compositor's `PixelCanvas` trait, including the framebuffer adapter.

## Features

- Allocation-free 5×7 bitmap glyphs in 6×9 character cells.
- Uppercase letters, digits, and common punctuation; lowercase ASCII is
  rendered as uppercase.
- Configurable foreground/background colors.
- Newline, carriage return, and tab handling.
- Bounds-aware drawing and cursor position reporting.
- Clear operation that fills the canvas and resets the cursor.

## Current limitations

This is a drawing primitive, not a replacement for the existing VGA/serial
kernel shell. It does not yet receive shell output, scroll text, or run as the
boot-time desktop console. Unicode and a full proportional font are not
supported.

Run the host-side tests with:

```sh
cargo test -p nexkernel --test graphics_console
```
