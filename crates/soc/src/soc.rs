use crate::header::Header;
use cpu::cpu::Cpu;
use cpu::Registers;
use memory::{Memory, NewMemory};
use pretty_hex::*;
use std::convert::TryFrom;
use std::fs;

const ROM_START: usize = 0x150;
const HEADER_START: usize = 0x100;
const HEADER_LEN: usize = 0x50;
const HEAD_LEN: usize = 0x100;

pub struct SOC /*<'a>*/ {
    //clock: u32,
    cpu: Cpu, /*<'a>*/
}

impl TryFrom<&str> for SOC /*<'a>*/ {
    type Error = std::io::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let mut head = fs::read(path)?;
        let rom = head.split_off(ROM_START);
        let raw_header = head.split_off(HEADER_START);

        assert_eq!(head.len(), HEAD_LEN);
        assert_eq!(raw_header.len(), HEADER_LEN);

        println!("head - {:?}", head.hex_dump());
        println!("header - {:?}", raw_header.hex_dump());

        let header = Header::try_from(raw_header).expect("Invalid data in raw_header");

        let _clock: u32 = 0;
        let _memory = <Memory as NewMemory>::new(header.cartridge, rom);
        let cpu: Cpu = Cpu::new(/*memory*/);

        Ok(SOC { /*clock,*/ cpu, })
    }
}

impl SOC /*<'a>*/ {
    pub fn get_cpu_registers(&self) -> Registers {
        self.cpu.get_registers()
    }
}
