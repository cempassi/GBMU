use crate::runner::Runner;
use crate::System;
use shared::Redraw;
use std::fs;

use crate::header::Header;
use memory;

const HEADER_START: usize = 0x100;
const HEADER_END: usize = 0x150;

/// The SOC is the GBMU async executor
pub struct SOC {
    status: System,
    processor: Runner,
}

impl TryFrom<&str> for SOC {
    type Error = std::io::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let rom = fs::read(path)?;
        let raw_header = rom[HEADER_START..HEADER_END].to_vec();

        let header = Header::try_from(raw_header).expect("Invalid data in raw_header");
        println!("Header: {:#?}", header);

        let state = memory::state::State::Bios;
        let memory: memory::Memory = memory::memory::Memory::new(header.cartridge, rom, state);
        let processor = Runner::new(memory, state);
        let status = System::new(processor.cpu());

        Ok(SOC { processor, status })
    }
}

impl SOC {
    pub fn get_ppu(&self) -> ppu::Ppu {
        self.processor.ppu()
    }

    pub fn get_cpu(&self) -> cpu::Cpu {
        self.processor.cpu()
    }

    pub fn get_memory(&self) -> memory::Memory {
        self.processor.memory.clone()
    }

    pub fn get_status(&self) -> System {
        self.status.clone()
    }

    pub fn run(&mut self) -> Redraw {
        let mut status = self.status.borrow_mut();
        status.redraw.clear();
        if status.is_idle() {
            return Redraw::Nope;
        }
        while status.processing() {
            status.step();
            let finished = self.processor.run();
            status.check_redraw(finished)
        }
        println!("[SOC] Finished Run. Redraw: {:?}", status.redraw);
        status.redraw
    }
}
