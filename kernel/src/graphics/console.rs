pub struct GraphicsConsole {
    pub width: usize,
    pub height: usize,
}

impl GraphicsConsole {
    pub const fn new(width: usize, height: usize) -> Self { Self { width, height } }
}
