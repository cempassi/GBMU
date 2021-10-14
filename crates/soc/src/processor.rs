use cpu::cpu::Cpu;
use memory::Memory;
use ppu::{Ppu, Run};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub type State = Option<Pin<Box<dyn Future<Output = u8>>>>;

pub enum Processor {
    Ppu(Ppu, State),
    Cpu(Cpu, State),
}

impl Processor {
    pub fn init(memory: Memory) -> Vec<Self> {
        let ppu = memory.borrow().get_ppu();
        let ppu = Processor::Ppu(ppu, None);
        let cpu = Processor::Cpu(Cpu::new(memory), None);
        vec![cpu, ppu]
    }

    pub fn run(&mut self, context: &mut Context) {
        match self {
            Processor::Ppu(ppu, ref mut state) => {
                if let Some(mut task) = state.take() {
                    match task.as_mut().poll(context) {
                        Poll::Ready(_) => state.replace(Box::pin(ppu.clone().run())),
                        Poll::Pending => state.replace(task),
                    };
                } else {
                    let mut task = Box::pin(ppu.clone().run());
                    match task.as_mut().poll(context) {
                        Poll::Pending => state.replace(task),
                        Poll::Ready(_) => state.replace(task),
                    };
                }
            }
            Processor::Cpu(cpu, ref mut state) => {
                if let Some(mut task) = state.take() {
                    match task.as_mut().poll(context) {
                        Poll::Ready(_) => state.replace(Box::pin(cpu.clone().run())),
                        Poll::Pending => state.replace(task),
                    };
                } else {
                    let mut task = Box::pin(cpu.clone().run());
                    match task.as_mut().poll(context) {
                        Poll::Pending => state.replace(task),
                        Poll::Ready(_) => state.replace(task),
                    };
                }
            }
        }
    }
}
