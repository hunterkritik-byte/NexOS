use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};

pub struct Framebuffer {
    buffer: FrameBuffer,
    info: FrameBufferInfo,
}

impl Framebuffer {
    pub fn new(buffer: FrameBuffer) -> Self {
        let info = buffer.info();
        Self { buffer, info }
    }

    pub fn info(&self) -> FrameBufferInfo { self.info }

    pub fn clear(&mut self, pixel: [u8; 4]) {
        let bytes = self.buffer.buffer_mut();
        let bpp = self.info.bytes_per_pixel;
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                let i = y * self.info.stride * bpp + x * bpp;
                if i + bpp > bytes.len() { continue; }
                match self.info.pixel_format {
                    PixelFormat::Rgb => {
                        bytes[i] = pixel[0]; if bpp > 1 { bytes[i+1]=pixel[1]; }
                        if bpp > 2 { bytes[i+2]=pixel[2]; }
                    }
                    PixelFormat::Bgr => {
                        bytes[i] = pixel[2]; if bpp > 1 { bytes[i+1]=pixel[1]; }
                        if bpp > 2 { bytes[i+2]=pixel[0]; }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, rgb: [u8;3]) {
        if x >= self.info.width || y >= self.info.height { return; }
        let bpp=self.info.bytes_per_pixel;
        let i=y*self.info.stride*bpp+x*bpp;
        let b=self.buffer.buffer_mut();
        if i+bpp>b.len(){return;}
        match self.info.pixel_format {
            PixelFormat::Rgb => { b[i]=rgb[0]; if bpp>1{b[i+1]=rgb[1];} if bpp>2{b[i+2]=rgb[2];} }
            PixelFormat::Bgr => { b[i]=rgb[2]; if bpp>1{b[i+1]=rgb[1];} if bpp>2{b[i+2]=rgb[0];} }
            _ => {}
        }
    }
}


impl super::compositor::PixelCanvas for Framebuffer {
    fn dimensions(&self) -> (usize, usize) {
        (self.info.width, self.info.height)
    }

    fn put_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        Framebuffer::put_pixel(self, x, y, color);
    }
}
