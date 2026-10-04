#[path = "../src/graphics/window.rs"]
mod window;
#[path = "../src/graphics/compositor.rs"]
mod compositor;
#[path = "../src/graphics/framebuffer.rs"]
mod framebuffer;

use compositor::PixelCanvas;

/// Compile-time contract check: the real framebuffer must be accepted by the
/// same renderer interface used by host-side test canvases.
fn assert_pixel_canvas<T: PixelCanvas>() {}

#[test]
fn boot_framebuffer_implements_compositor_canvas() {
    assert_pixel_canvas::<framebuffer::Framebuffer>();
}
