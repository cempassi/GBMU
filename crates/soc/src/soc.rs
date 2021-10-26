use super::processor::Finished;
use crate::processor::Processor;
use crate::runner::Runner;
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
    runner: Runner,
    processors: Vec<Processor>,
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
        let runner = Runner::default();

        Ok(SOC { processors, runner })
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

    pub fn get_ppu(&self) -> ppu::Ppu {
        self.processors
            .iter()
            .find_map(|x| {
                if let Processor::Ppu(ppu, _) = x {
                    Some(ppu.clone())
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

    pub fn get_runner(&self) -> Runner {
        self.runner.clone()
    }

    fn check_tick(&mut self) -> bool {
        self.runner.borrow_mut().check_tick()
    }

    fn check_redraw(&mut self, status: Vec<Finished>) -> bool {
        self.runner.borrow_mut().check_redraw(status)
    }

    pub fn run(&mut self) -> bool {
        let waker = crate::waker::create();
        let mut context = Context::from_waker(&waker);
        let mut status = Vec::new();

        if self.check_tick() {
            for processor in &mut self.processors {
                status.push(processor.run(&mut context));
            }
        }
        self.check_redraw(status)
    }
}
