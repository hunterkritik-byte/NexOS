use super::compositor::{Color, PixelCanvas};

const GLYPH_WIDTH: usize = 5;
const GLYPH_HEIGHT: usize = 7;
const CELL_WIDTH: usize = 6;
const CELL_HEIGHT: usize = 9;

/// A tiny allocation-free text console for framebuffer-backed canvases.
/// The built-in font covers uppercase Latin letters, digits, and common
/// punctuation; lowercase ASCII is rendered as uppercase.
pub struct GraphicsConsole {
    pub width: usize,
    pub height: usize,
    cursor_x: usize,
    cursor_y: usize,
    foreground: Color,
    background: Color,
}

impl GraphicsConsole {
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
            foreground: [240, 240, 240],
            background: [24, 32, 48],
        }
    }

    pub fn set_colors(&mut self, foreground: Color, background: Color) {
        self.foreground = foreground;
        self.background = background;
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn clear<C: PixelCanvas>(&mut self, canvas: &mut C) {
        let (canvas_width, canvas_height) = canvas.dimensions();
        let width = self.width.min(canvas_width);
        let height = self.height.min(canvas_height);
        for y in 0..height {
            for x in 0..width {
                canvas.put_pixel(x, y, self.background);
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    pub fn write_text<C: PixelCanvas>(&mut self, canvas: &mut C, text: &str) {
        let (canvas_width, canvas_height) = canvas.dimensions();
        let width = self.width.min(canvas_width);
        let height = self.height.min(canvas_height);

        for byte in text.bytes() {
            match byte {
                b'\n' => self.new_line(width, height),
                b'\r' => self.cursor_x = 0,
                b'\t' => {
                    let next = (self.cursor_x / (CELL_WIDTH * 4) + 1)
                        .saturating_mul(CELL_WIDTH * 4);
                    self.cursor_x = next.min(width);
                }
                byte => {
                    if self.cursor_x.saturating_add(GLYPH_WIDTH) > width {
                        self.new_line(width, height);
                    }
                    if self.cursor_y.saturating_add(GLYPH_HEIGHT) > height {
                        self.cursor_y = 0;
                    }
                    let rows = glyph(byte.to_ascii_uppercase());
                    for (row, bits) in rows.iter().enumerate() {
                        for col in 0..GLYPH_WIDTH {
                            let x = self.cursor_x.saturating_add(col);
                            let y = self.cursor_y.saturating_add(row);
                            if x < width && y < height {
                                let mask = 1 << (GLYPH_WIDTH - 1 - col);
                                canvas.put_pixel(
                                    x,
                                    y,
                                    if bits & mask != 0 { self.foreground } else { self.background },
                                );
                            }
                        }
                    }
                    self.cursor_x = self.cursor_x.saturating_add(CELL_WIDTH);
                }
            }
        }
    }

    fn new_line(&mut self, width: usize, height: usize) {
        self.cursor_x = 0;
        self.cursor_y = self.cursor_y.saturating_add(CELL_HEIGHT);
        if self.cursor_y.saturating_add(GLYPH_HEIGHT) > height {
            self.cursor_y = 0;
        }
        if width == 0 || height == 0 {
            self.cursor_x = 0;
            self.cursor_y = 0;
        }
    }
}

impl Default for GraphicsConsole {
    fn default() -> Self { Self::new(640, 480) }
}

fn glyph(byte: u8) -> [u8; GLYPH_HEIGHT] {
    match byte {
        b'A' => [14, 17, 17, 31, 17, 17, 17],
        b'B' => [30, 17, 17, 30, 17, 17, 30],
        b'C' => [14, 17, 16, 16, 16, 17, 14],
        b'D' => [30, 17, 17, 17, 17, 17, 30],
        b'E' => [31, 16, 16, 30, 16, 16, 31],
        b'F' => [31, 16, 16, 30, 16, 16, 16],
        b'G' => [14, 17, 16, 23, 17, 17, 15],
        b'H' => [17, 17, 17, 31, 17, 17, 17],
        b'I' => [14, 4, 4, 4, 4, 4, 14],
        b'J' => [7, 2, 2, 2, 18, 18, 12],
        b'K' => [17, 18, 20, 24, 20, 18, 17],
        b'L' => [16, 16, 16, 16, 16, 16, 31],
        b'M' => [17, 27, 21, 21, 17, 17, 17],
        b'N' => [17, 25, 21, 19, 17, 17, 17],
        b'O' => [14, 17, 17, 17, 17, 17, 14],
        b'P' => [30, 17, 17, 30, 16, 16, 16],
        b'Q' => [14, 17, 17, 17, 21, 18, 13],
        b'R' => [30, 17, 17, 30, 20, 18, 17],
        b'S' => [15, 16, 16, 14, 1, 1, 30],
        b'T' => [31, 4, 4, 4, 4, 4, 4],
        b'U' => [17, 17, 17, 17, 17, 17, 14],
        b'V' => [17, 17, 17, 17, 17, 10, 4],
        b'W' => [17, 17, 17, 21, 21, 21, 10],
        b'X' => [17, 17, 10, 4, 10, 17, 17],
        b'Y' => [17, 17, 10, 4, 4, 4, 4],
        b'Z' => [31, 1, 2, 4, 8, 16, 31],
        b'0' => [14, 17, 19, 21, 25, 17, 14],
        b'1' => [4, 12, 4, 4, 4, 4, 14],
        b'2' => [14, 17, 1, 2, 4, 8, 31],
        b'3' => [30, 1, 1, 14, 1, 1, 30],
        b'4' => [2, 6, 10, 18, 31, 2, 2],
        b'5' => [31, 16, 16, 30, 1, 1, 30],
        b'6' => [14, 16, 16, 30, 17, 17, 14],
        b'7' => [31, 1, 2, 4, 8, 8, 8],
        b'8' => [14, 17, 17, 14, 17, 17, 14],
        b'9' => [14, 17, 17, 15, 1, 1, 14],
        b' ' => [0; GLYPH_HEIGHT],
        b'.' => [0, 0, 0, 0, 0, 6, 6],
        b':' => [0, 6, 6, 0, 6, 6, 0],
        b'-' => [0, 0, 0, 31, 0, 0, 0],
        b'_' => [0, 0, 0, 0, 0, 0, 31],
        b'/' => [1, 1, 2, 4, 8, 16, 16],
        b'!' => [4, 4, 4, 4, 4, 0, 4],
        b'?' => [14, 17, 1, 2, 4, 0, 4],
        _ => [31, 17, 5, 9, 4, 0, 4],
    }
}
