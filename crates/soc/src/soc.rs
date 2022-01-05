use crate::runner::Runner;
use crate::System;
use shared::Redraw;
use std::fs;

use memory;
use memory::header::Header;

const HEADER_START: usize = 0x100;
const HEADER_END: usize = 0x150;

/// The SOC is the GBMU async executor
pub struct SOC {
    status: System,
    ppu: ppu::Ppu,
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
        let memory: memory::Memory = memory::memory::Memory::new(header, rom, state);
        let ppu = memory.borrow().get_ppu();
        let processor = Runner::new(memory, state);
        let status = System::new(processor.cpu());

        Ok(SOC { processor, ppu, status })
    }
}

impl SOC {
    pub fn get_cpu(&self) -> cpu::Cpu {
        self.processor.cpu()
    }

    pub fn ppu(&self) -> ppu::Ppu {
        self.ppu.clone()
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
        // if status.is_idle() {
        //     return Redraw::Nope;
        // }
        // //while status.processing() {
           // status.step();
        let finished = self.processor.run();
            //status.check_redraw(finished)
        //}
        //println!("[SOC] Finished Run. Redraw: {:?}", status.redraw);
        //status.redraw
        Redraw::All
    }
}
