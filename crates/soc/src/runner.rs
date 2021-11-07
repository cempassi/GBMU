use cpu::Cpu;
use memory::Memory;
use ppu::Ppu;
use shared::{Finished, Output, Run};
use std::task::{Context, Poll};

pub struct Runner {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub memory: Memory,
    tasks: Tasks,
}

impl Runner {
    pub fn new(memory: Memory) -> Self {
        let ppu = memory.borrow().get_ppu();
        let cpu = Cpu::new(memory.clone());
        let tasks = Tasks::new(cpu.clone().run(), ppu.clone().run());
        Self {
            ppu,
            cpu,
            memory,
            tasks,
        }
    }

    pub fn run(&mut self) -> Vec<Finished> {
        let waker = shared::waker::create();
        let mut context = Context::from_waker(&waker);

        let cpu_status = self.tasks.run_cpu(self.cpu.clone(), &mut context);
        let ppu_status = self.tasks.run_ppu(self.ppu.clone(), &mut context);
        vec![cpu_status, ppu_status]
    }
}

struct Tasks {
    cpu: Output,
    ppu: Output,
}

impl Tasks {
    pub fn new(cpu: Output, ppu: Output) -> Self {
        Self { cpu, ppu }
    }

    fn run_cpu(&mut self, processor: impl Run, context: &mut Context) -> Finished {
        match self.cpu.as_mut().poll(context) {
            Poll::Ready(status) => {
                self.cpu = processor.run();
                Finished::finish(status)
            }
            Poll::Pending => Finished::Nope,
        }
    }

    fn run_ppu(&mut self, processor: impl Run, context: &mut Context) -> Finished {
        match self.ppu.as_mut().poll(context) {
            Poll::Ready(status) => {
                self.ppu = processor.run();
                Finished::finish(status)
            }
            Poll::Pending => Finished::Nope,
        }
    }
}
