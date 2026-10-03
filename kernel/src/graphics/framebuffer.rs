use bootloader_api::info::FrameBuffer;

pub struct Framebuffer {
    buffer: FrameBuffer,
}

impl Framebuffer {
    pub fn new(buffer: FrameBuffer) -> Self { Self { buffer } }

    pub fn info(&self) -> bootloader_api::info::FrameBufferInfo {
        self.buffer.info()
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer.buffer_mut()
    }
}
