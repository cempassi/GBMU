use crate::processor::Processor;
use std::convert::TryFrom;
use std::fs;
use std::task::Context;

use crate::header::Header;
use cpu::Registers;
use memory;

const ROM_START: usize = 0x150;
const HEADER_START: usize = 0x100;
const HEADER_LEN: usize = 0x50;
const HEAD_LEN: usize = 0x100;

/// The SOC is the GBMU async executor
pub struct SOC {
    processors: Vec<Processor>,
    clock: u32,
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
        let processors = Processor::init(memory);
        let clock = 0;

        Ok(SOC { processors, clock })
    }
}

impl SOC {
    pub fn get_cpu_registers(&self) -> Registers {
        self.processors
            .iter()
            .find_map(|x| {
                if let Processor::Cpu(cpu, _) = x {
                    Some(cpu.get_registers())
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn get_memory(&self) -> memory::Memory {
        self.processors
            .iter()
            .find_map(|x| {
                if let Processor::Cpu(cpu, _) = x {
                    Some(cpu.get_memory())
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn run(&mut self) {
        let waker = crate::waker::create();
        let mut context = Context::from_waker(&waker);

        if self.clock < 20 {
            self.clock += 1;
            for processor in &mut self.processors {
                processor.run(&mut context);
            }
        }
    }
}
