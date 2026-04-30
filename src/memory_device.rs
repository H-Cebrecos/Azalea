#[derive(Copy, Clone)]
pub enum AccessSize {
    Byte,
    Half,
    Word,
}

pub trait MemoryDevice {
    fn contains_addr(&self, addr: u32) -> bool;
    fn load(&mut self, addr: u32, size: AccessSize) -> u32;
    fn store(&mut self, addr: u32, size: AccessSize, value: u32);
}
