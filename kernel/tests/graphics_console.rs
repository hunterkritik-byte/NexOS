#[path = "../src/graphics/window.rs"]
mod window;
#[path = "../src/graphics/compositor.rs"]
mod compositor;
#[path = "../src/graphics/console.rs"]
mod console;

use compositor::PixelCanvas;
use console::GraphicsConsole;

struct TestCanvas {
    width: usize,
    height: usize,
    pixels: Vec<[u8; 3]>,
}

impl TestCanvas {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: vec![[0, 0, 0]; width * height] }
    }

    fn pixel(&self, x: usize, y: usize) -> [u8; 3] {
        self.pixels[y * self.width + x]
    }
}

impl PixelCanvas for TestCanvas {
    fn dimensions(&self) -> (usize, usize) { (self.width, self.height) }

    fn put_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
}

#[test]
fn writes_a_glyph_with_foreground_and_background_pixels() {
    let mut canvas = TestCanvas::new(20, 10);
    let mut console = GraphicsConsole::new(20, 10);
    console.set_colors([255, 0, 0], [0, 0, 0]);

    console.write_text(&mut canvas, "A");

    assert_eq!(canvas.pixel(0, 0), [0, 0, 0]);
    assert_eq!(canvas.pixel(1, 0), [255, 0, 0]);
    assert_eq!(console.cursor_position(), (6, 0));
}

#[test]
fn lowercase_is_rendered_as_uppercase_and_newline_advances() {
    let mut canvas = TestCanvas::new(40, 20);
    let mut console = GraphicsConsole::new(40, 20);
    console.set_colors([255, 255, 255], [0, 0, 0]);

    console.write_text(&mut canvas, "a\nB");

    assert_eq!(canvas.pixel(1, 0), [255, 255, 255]);
    assert_eq!(canvas.pixel(1, 9), [255, 255, 255]);
    assert_eq!(console.cursor_position(), (6, 9));
}

#[test]
fn clear_fills_canvas_and_resets_cursor() {
    let mut canvas = TestCanvas::new(12, 9);
    let mut console = GraphicsConsole::new(12, 9);
    console.set_colors([255, 255, 255], [3, 4, 5]);
    console.write_text(&mut canvas, "A");
    console.clear(&mut canvas);

    assert!(canvas.pixels.iter().all(|pixel| *pixel == [3, 4, 5]));
    assert_eq!(console.cursor_position(), (0, 0));
}

#[test]
fn writes_are_clipped_to_console_and_canvas_dimensions() {
    let mut canvas = TestCanvas::new(7, 7);
    let mut console = GraphicsConsole::new(100, 100);
    console.write_text(&mut canvas, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");

    assert_eq!(canvas.pixels.len(), 49);
    assert_eq!(console.cursor_position().1, 0);
}
