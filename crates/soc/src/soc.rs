use crate::header::Header;
use cpu::cpu::Cpu;
use cpu::Registers;
use memory;
use pretty_hex::*;
use std::convert::TryFrom;
use std::fs;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

const ROM_START: usize = 0x150;
const HEADER_START: usize = 0x100;
const HEADER_LEN: usize = 0x50;
const HEAD_LEN: usize = 0x100;

/// The SOC is the GBMU async executor
pub struct SOC {
    cpu: Cpu,
    state: Option<Pin<Box<dyn Future<Output = u8>>>>,
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

        println!("head - {:?}", head.hex_dump());
        println!("header - {:?}", raw_header.hex_dump());

        let header = Header::try_from(raw_header).expect("Invalid data in raw_header");

        let memory: memory::Memory = memory::memory::Memory::new(header.cartridge, rom);
        let cpu: Cpu = Cpu::new(memory);
        let state = None;
        let clock = 0;

        Ok(SOC { clock, cpu, state })
    }
}

impl SOC {
    pub fn get_cpu_registers(&self) -> Registers {
        self.cpu.get_registers()
    }

    pub fn get_memory(&self) -> memory::Memory {
        self.cpu.get_memory()
    }

    fn run_cpu(&mut self, context: &mut Context) {
        if let Some(mut task) = self.state.take() {
            self.state = match task.as_mut().poll(context) {
                Poll::Ready(_) => Some(Box::pin(self.cpu.clone().run())),
                Poll::Pending => Some(task),
            };
        } else {
            let mut task = Box::pin(self.cpu.clone().run());
            self.state = match task.as_mut().poll(context) {
                Poll::Pending => Some(task),
                Poll::Ready(_) => None,
            }
        }
    }

    pub fn run(&mut self) {
        let waker = crate::waker::create();
        let mut context = Context::from_waker(&waker);
        if self.clock < 20 {
            self.run_cpu(&mut context);
            self.clock += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Executing {
    Processing,
    Idle,
    Finished(u8),
}

