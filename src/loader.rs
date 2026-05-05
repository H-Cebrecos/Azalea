use elftool::{
    elf::{ElfErr, ElfReader, segment::SegmentType},
    reader::ReaderCtx,
};

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub struct FileReader {
    file: File,
}

impl FileReader {
    pub fn open(path: &str) -> Result<Self, ElfErr> {
        let file = File::open(path).map_err(|_| ElfErr::IoError)?;
        Ok(Self { file })
    }
}

impl ElfReader for FileReader {
    fn read(&mut self, offset: u64, buf: &mut [u8]) -> Result<(), ElfErr> {
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|_| ElfErr::IoError)?;

        self.file.read_exact(buf).map_err(|_| ElfErr::IoError)?;

        Ok(())
    }
}

use crate::memory_device::{AccessSize, MemoryDevice};
pub fn load(path: &str, mem: &mut impl MemoryDevice) -> u32 {
    let file = File::open(path).unwrap();

    let mut elf = ReaderCtx::new(FileReader { file }).unwrap();

    let hdr = elf.get_hdr();

    for i in 0..hdr.ph_entry_num {
        let ph = elf.get_program_hdr(i).unwrap();

        if ph.seg_type != SegmentType::Load {
            continue;
        }

        let mut data = vec![0u8; ph.file_size as usize];

        elf.read_at(ph.offset, &mut data).unwrap();

        for (i, byte) in data.iter().enumerate() {
            mem.store(
                ph.virt_addr as u32 + i as u32,
                AccessSize::Byte,
                *byte as u32,
            );
        }

        for i in ph.file_size..ph.mem_size {
            mem.store(ph.virt_addr as u32 + i as u32, AccessSize::Byte, 0);
        }
    }

    return elf.get_hdr().entry as u32;
}
