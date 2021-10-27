use cpu::cpu::Cpu;
use memory::Memory;
use ppu::{Ppu, Run};
use shared::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub type State = Option<Pin<Box<dyn Future<Output = Result<u8, Error>>>>>;

pub enum Processor {
    Ppu(Ppu, State),
    Cpu(Cpu, State),
}

pub enum Finished {
    Cpu(u8),
    Ppu(u8),
    Error(Error),
    Nope,
}

impl Finished {
    pub fn ppu(result: Result<u8, Error>) -> Self {
        match result {
            Ok(cycles) => {
                println!("PPU finised, cycles: {}", cycles);
                Self::Ppu(cycles)
            }
            Err(error) => Self::Error(error),
        }
    }
    pub fn cpu(result: Result<u8, Error>) -> Self {
        match result {
            Ok(cycles) => {
                println!("CPU finished, cycles: {}", cycles);
                Self::Cpu(cycles)
            }
            Err(error) => {
                println!("CPU ERROR, error: {}", error);
                Self::Error(error)
            }
        }
    }
}

impl Processor {
    pub fn init(memory: Memory) -> Vec<Self> {
        let ppu = memory.borrow().get_ppu();
        let ppu = Processor::Ppu(ppu, None);
        let cpu = Processor::Cpu(Cpu::new(memory), None);
        vec![cpu, ppu]
    }

    pub fn run(&mut self, context: &mut Context) -> Finished {
        match self {
            Processor::Ppu(ppu, ref mut state) => {
                if let Some(mut task) = state.take() {
                    match task.as_mut().poll(context) {
                        Poll::Ready(status) => {
                            state.replace(Box::pin(ppu.clone().run()));
                            Finished::ppu(status)
                        }
                        Poll::Pending => {
                            state.replace(task);
                            Finished::Nope
                        }
                    }
                } else {
                    let mut task = Box::pin(ppu.clone().run());
                    match task.as_mut().poll(context) {
                        Poll::Pending => {
                            state.replace(task);
                            Finished::Nope
                        }
                        Poll::Ready(status) => {
                            state.replace(Box::pin(ppu.clone().run()));
                            Finished::ppu(status)
                        }
                    }
                }
            }
            Processor::Cpu(cpu, ref mut state) => {
                if let Some(mut task) = state.take() {
                    match task.as_mut().poll(context) {
                        Poll::Ready(status) => {
                            state.replace(Box::pin(cpu.clone().run()));
                            Finished::cpu(status)
                        }
                        Poll::Pending => {
                            state.replace(task);
                            Finished::Nope
                        }
                    }
                } else {
                    let mut task = Box::pin(cpu.clone().run());
                    match task.as_mut().poll(context) {
                        Poll::Pending => {
                            state.replace(task);
                            Finished::Nope
                        }
                        Poll::Ready(status) => {
                            state.replace(Box::pin(cpu.clone().run()));
                            Finished::cpu(status)
                        }
                    }
                }
            }
        }
    }
}
