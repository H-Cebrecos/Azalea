use super::*;

pub struct Bus {
    devices: Vec<Box<dyn MemoryDevice>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, dev: impl MemoryDevice + 'static) {
        self.devices.push(Box::new(dev));
    }

    fn find_device(&mut self, addr: u32) -> &mut dyn MemoryDevice {
        let dev = self.devices.iter_mut().find(|d| d.contains_addr(addr));

        match dev {
            Some(d) => d.as_mut(),
            None => {
                eprintln!("invalid memory access at 0x{:08x}", addr);
                panic!("no device mapped for address");
            }
        }
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
