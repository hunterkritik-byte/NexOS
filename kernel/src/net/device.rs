pub trait NetworkDevice {
    fn mac_address(&self) -> [u8;6];
    fn transmit(&mut self, packet:&[u8]) -> Result<(), &'static str>;
    fn receive(&mut self, buffer:&mut [u8]) -> Result<usize, &'static str>;
    fn link_up(&self) -> bool;
}
