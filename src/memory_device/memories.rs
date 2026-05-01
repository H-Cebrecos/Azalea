use super::*;

pub struct Ram {
    start: u32,
    data: Vec<u8>,
}

impl Ram {
    pub fn new(start_addr: u32, size: usize) -> Self {
        Self {
            start: start_addr,
            data: vec![0; size],
        }
    }
}

impl MemoryDevice for Ram {
    fn load(&mut self, offset: u32, size: AccessSize) -> u32 {
        let i = offset as usize;

        match size {
            AccessSize::Byte => self.data[i] as u32,
            AccessSize::Half => {
                let bytes: [u8; 2] = self.data[i..i + 2].try_into().unwrap();
                u16::from_le_bytes(bytes) as u32
            }
            AccessSize::Word => {
                let bytes: [u8; 4] = self.data[i..i + 4].try_into().unwrap();
                u32::from_le_bytes(bytes)
            }
        }
    }

    fn store(&mut self, offset: u32, size: AccessSize, value: u32) {
        let i = offset as usize;

        match size {
            AccessSize::Byte => {
                self.data[i] = value as u8;
            }
            AccessSize::Half => {
                self.data[i..i + 2].copy_from_slice(&(value as u16).to_le_bytes());
            }
            AccessSize::Word => {
                self.data[i..i + 4].copy_from_slice(&value.to_le_bytes());
            }
        }
    }

    fn contains_addr(&self, addr: u32) -> bool {
        addr >= self.start && addr < self.start + self.data.len() as u32
    }
}

pub struct Rom {
    start: u32,
    data: Vec<u8>,
}

impl Rom {
    pub fn new(start_addr: u32, data: &[u8]) -> Self {
        Self {
            start: start_addr,
            data: data.into(),
        }
    }
}

impl MemoryDevice for Rom {
    fn load(&mut self, offset: u32, size: AccessSize) -> u32 {
        let i = offset as usize;

        match size {
            AccessSize::Byte => self.data[i] as u32,
            AccessSize::Half => {
                let bytes: [u8; 2] = self.data[i..i + 2].try_into().unwrap();
                u16::from_le_bytes(bytes) as u32
            }
            AccessSize::Word => {
                let bytes: [u8; 4] = self.data[i..i + 4].try_into().unwrap();
                u32::from_le_bytes(bytes)
            }
        }
    }

    fn store(&mut self, offset: u32, size: AccessSize, value: u32) {
        _ = offset;
        _ = size;
        _ = value;
    }

    fn contains_addr(&self, addr: u32) -> bool {
        addr >= self.start && addr < self.start + self.data.len() as u32
    }
}
