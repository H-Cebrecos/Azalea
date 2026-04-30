use crate::memory_device::*;

pub struct Bus {
    devices: Vec<Box<dyn MemoryDevice>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, dev: Box<dyn MemoryDevice>) {
        self.devices.push(dev);
    }

    fn find_device(&mut self, addr: u32) -> &mut dyn MemoryDevice {
        self.devices
            .iter_mut()
            .find(|d| d.contains_addr(addr))
            .expect("no device mapped for address")
            .as_mut()
    }
}

impl MemoryDevice for Bus {
    fn contains_addr(&self, addr: u32) -> bool {
        self.devices.iter().any(|d| d.contains_addr(addr))
    }

    fn load(&mut self, addr: u32, size: AccessSize) -> u32 {
        self.find_device(addr).load(addr, size)
    }

    fn store(&mut self, addr: u32, size: AccessSize, value: u32) {
        self.find_device(addr).store(addr, size, value)
    }
}
