use crate::header::Header;
use cpu::cpu::Cpu;
use memory::Memory;
use pretty_hex::*;
use std::convert::TryFrom;
use std::fs;

const ROM_START: usize = 0x150;
const HEADER_START: usize = 0x100;

pub struct SOC {
    clock: u32,
    cpu: Cpu,
    memory: Memory,
}

impl Default for SOC {
    fn default() -> Self {
        SOC::new()
    }
}

impl TryFrom<&str> for SOC {
    type Error = std::io::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let mut head = fs::read(path)?;
        let _rom = head.split_off(ROM_START);
        let raw_header = head.split_off(HEADER_START);
        assert_eq!(head.len(), 0x100);
        assert_eq!(raw_header.len(), 0x50);

        println!("head - {:?}", head.hex_dump());
        println!("header - {:?}", raw_header.hex_dump());
        let header = Header::try_from(raw_header).expect("Invalid data in raw_header");
        println!("{:#x?}", header);
        Ok(SOC::new())
    }
}

impl SOC {
    pub fn new() -> Self {
        SOC {
            clock: 0,
            cpu: Cpu::default(),
            memory: Memory::default(),
        }
    }

    pub fn run(&mut self) {
        match self.cpu.step(&mut self.memory) {
            Ok(cycles) => self.clock += cycles,
            Err(_) => (),
        }
    }
}
