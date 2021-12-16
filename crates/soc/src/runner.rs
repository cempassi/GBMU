use cpu::Cpu;
use memory::Memory;
use ppu::Ppu;
use shared::{Finished, Output, Run};
use std::task::{Context, Poll};

enum Processor {
    Ppu,
    Cpu,
}

pub struct Runner {
    pub memory: Memory,
    tasks: Tasks,
}

impl Runner {
    pub fn new(memory: Memory, state: memory::State) -> Self {
        let ppu = memory.borrow().get_ppu();
        let cpu = match state {
            memory::State::Bios => Cpu::new(memory.clone()),
            memory::State::Rom => Cpu::no_bios(memory.clone()),
        };
        let tasks = Tasks::new(cpu, ppu);
        Self { memory, tasks }
    }

    pub fn run(&mut self) -> Vec<Finished> {
        let waker = shared::waker::create();
        let mut context = Context::from_waker(&waker);

        self.memory.borrow_mut().clock_tick();
        let cpu_status = self.tasks.run(Processor::Cpu, &mut context);
        let ppu_status = self.tasks.run(Processor::Ppu, &mut context);
        vec![cpu_status, ppu_status]
    }

    pub fn cpu(&self) -> Cpu {
        self.tasks.cpu.clone()
    }

    pub fn ppu(&self) -> Ppu {
        self.tasks.ppu.clone()
    }
}

struct Tasks {
    cpu: Cpu,
    ppu: Ppu,
    cpu_process: Output,
    ppu_process: Output,
}

impl Tasks {
    pub fn new(cpu: Cpu, ppu: Ppu) -> Self {
        let cpu_process = cpu.clone().run();
        let ppu_process = ppu.clone().run();
        Self {
            cpu,
            ppu,
            cpu_process,
            ppu_process,
        }
    }

    fn run(&mut self, processor: Processor, context: &mut Context) -> Finished {
        match processor {
            Processor::Ppu => match self.ppu_process.as_mut().poll(context) {
                Poll::Ready(status) => {
                    self.ppu_process = self.ppu.clone().run();
                    Finished::finish(status)
                }
                Poll::Pending => Finished::Nope,
            },
            Processor::Cpu => match self.cpu_process.as_mut().poll(context) {
                Poll::Ready(status) => {
                    self.cpu_process = self.cpu.clone().run();
                    Finished::finish(status)
                }
                Poll::Pending => Finished::Nope,
            },
        }
    }
}
