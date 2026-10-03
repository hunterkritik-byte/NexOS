#[path = "../src/graphics/window.rs"]
mod window;
#[path = "../src/graphics/compositor.rs"]
mod compositor;

use compositor::{render, PixelCanvas, Theme};
use window::{Rect, WindowManager};

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
fn compositor_paints_desktop_and_window_chrome() {
    let mut manager = WindowManager::new();
    let id = manager.create(Rect { x: 2, y: 3, width: 12, height: 10 }).unwrap();
    let mut canvas = TestCanvas::new(20, 16);
    let theme = Theme::default();

    render(&mut canvas, &manager, theme);

    assert_eq!(canvas.pixel(0, 0), theme.desktop);
    assert_eq!(canvas.pixel(2, 3), theme.border);
    assert_eq!(canvas.pixel(3, 4), theme.title_active);
    assert_eq!(canvas.pixel(4, 8), theme.window_body);

    assert!(manager.focus(id));
    render(&mut canvas, &manager, theme);
    assert_eq!(canvas.pixel(3, 4), theme.title_active);
}

#[test]
fn later_windows_paint_over_earlier_windows() {
    let mut manager = WindowManager::new();
    let _back = manager.create(Rect { x: 1, y: 1, width: 10, height: 10 }).unwrap();
    let front = manager.create(Rect { x: 4, y: 4, width: 10, height: 10 }).unwrap();
    let mut canvas = TestCanvas::new(16, 16);

    render(&mut canvas, &manager, Theme::default());
    assert_eq!(canvas.pixel(5, 5), Theme::default().title_active);

    assert!(manager.focus(_back));
    render(&mut canvas, &manager, Theme::default());
    assert_eq!(canvas.pixel(5, 5), Theme::default().title_inactive);
    assert_eq!(manager.window_at(5, 5), Some(_back));
    assert_ne!(front, _back);
}

#[test]
fn offscreen_and_partially_clipped_windows_are_safe() {
    let mut manager = WindowManager::new();
    manager.create(Rect { x: 7, y: 7, width: usize::MAX, height: usize::MAX });
    manager.create(Rect { x: 50, y: 50, width: 5, height: 5 });
    let mut canvas = TestCanvas::new(8, 8);

    render(&mut canvas, &manager, Theme::default());
    assert_eq!(canvas.pixel(7, 7), Theme::default().border);
    assert_eq!(canvas.pixel(0, 0), Theme::default().desktop);
}
