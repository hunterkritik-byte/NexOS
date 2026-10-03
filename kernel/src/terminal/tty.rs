use core::fmt::{self, Write};

pub struct Terminal<W: Write> {
    writer: W,
}

impl<W: Write> Terminal<W> {
    pub const fn new(writer: W) -> Self { Self { writer } }

    pub fn write_str(&mut self, s: &str) {
        let _ = self.writer.write_str(s);
    }
}

impl<W: Write> Write for Terminal<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}
