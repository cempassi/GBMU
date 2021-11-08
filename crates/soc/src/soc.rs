use crate::runner::Runner;
use crate::Status;
use shared::Redraw;
use std::fs;

use crate::header::Header;
use memory;

const ROM_START: usize = 0x150;
const HEADER_START: usize = 0x100;
const HEADER_LEN: usize = 0x50;
const HEAD_LEN: usize = 0x100;

/// The SOC is the GBMU async executor
pub struct SOC {
    status: Status,
    processor: Runner,
}

impl TryFrom<&str> for SOC {
    type Error = std::io::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let mut head = fs::read(path)?;
        let rom = head.split_off(ROM_START);
        let raw_header = head.split_off(HEADER_START);

        assert_eq!(head.len(), HEAD_LEN);
        assert_eq!(raw_header.len(), HEADER_LEN);

        let header = Header::try_from(raw_header).expect("Invalid data in raw_header");
        println!("Header: {:#?}", header);

        let memory: memory::Memory = memory::memory::Memory::new(header.cartridge, rom);
        let processor = Runner::new(memory);
        let runner = Status::default();

        Ok(SOC {
            processor,
            status: runner,
        })
    }
}

impl SOC {
    pub fn get_ppu(&self) -> ppu::Ppu {
        self.processor.ppu.clone()
    }

    pub fn get_cpu(&self) -> cpu::Cpu {
        self.processor.cpu.clone()
    }

    pub fn get_memory(&self) -> memory::Memory {
        self.processor.memory.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn step(&mut self) {
        self.status.borrow_mut().step()
    }

    pub fn run(&mut self) -> Redraw {
        if self.status.borrow().is_idle() {
            return Redraw::Nope;
        }
        while !self.status.borrow().is_idle() {
            self.step();
            let status = &mut self.processor.run();
            self.status.borrow_mut().check_redraw(status)
        }
        self.status.borrow().redraw
    }
}
