use super::window::{Rect, WindowManager};

pub type Color = [u8; 3];

/// Minimal drawing interface implemented by a framebuffer or a host test canvas.
pub trait PixelCanvas {
    fn dimensions(&self) -> (usize, usize);
    fn put_pixel(&mut self, x: usize, y: usize, color: Color);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Theme {
    pub desktop: Color,
    pub window_body: Color,
    pub title_active: Color,
    pub title_inactive: Color,
    pub border: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            desktop: [24, 32, 48],
            window_body: [232, 235, 240],
            title_active: [40, 105, 190],
            title_inactive: [85, 96, 112],
            border: [12, 16, 24],
        }
    }
}

/// Clears the canvas and paints rectangular window chrome back-to-front.
///
/// This is intentionally a small software compositor foundation: it paints
/// simple window rectangles and title bars, but does not draw text, buttons,
/// cursors, or application contents.
pub fn render<C: PixelCanvas>(canvas: &mut C, manager: &WindowManager, theme: Theme) {
    let (width, height) = canvas.dimensions();
    fill_rect(
        canvas,
        Rect {
            x: 0,
            y: 0,
            width,
            height,
        },
        theme.desktop,
    );

    for (_, window) in manager.z_ordered() {
        let bounds = clip_rect(window.bounds, width, height);
        if bounds.width == 0 || bounds.height == 0 {
            continue;
        }

        fill_rect(canvas, bounds, theme.border);
        if bounds.width > 2 && bounds.height > 2 {
            fill_rect(
                canvas,
                Rect {
                    x: bounds.x + 1,
                    y: bounds.y + 1,
                    width: bounds.width - 2,
                    height: bounds.height - 2,
                },
                theme.window_body,
            );
        }

        let title_height = core::cmp::min(20, bounds.height.saturating_sub(2));
        if title_height > 0 && bounds.width > 2 {
            fill_rect(
                canvas,
                Rect {
                    x: bounds.x + 1,
                    y: bounds.y + 1,
                    width: bounds.width - 2,
                    height: title_height,
                },
                if window.focused {
                    theme.title_active
                } else {
                    theme.title_inactive
                },
            );
        }
    }
}

fn clip_rect(rect: Rect, width: usize, height: usize) -> Rect {
    if rect.x >= width || rect.y >= height {
        return Rect {
            x: rect.x,
            y: rect.y,
            width: 0,
            height: 0,
        };
    }
    Rect {
        x: rect.x,
        y: rect.y,
        width: core::cmp::min(rect.width, width - rect.x),
        height: core::cmp::min(rect.height, height - rect.y),
    }
}

fn fill_rect<C: PixelCanvas>(canvas: &mut C, rect: Rect, color: Color) {
    let (width, height) = canvas.dimensions();
    if rect.width == 0 || rect.height == 0 || rect.x >= width || rect.y >= height {
        return;
    }
    let right = rect.x.saturating_add(rect.width).min(width);
    let bottom = rect.y.saturating_add(rect.height).min(height);
    for y in rect.y..bottom {
        for x in rect.x..right {
            canvas.put_pixel(x, y, color);
        }
    }
}
