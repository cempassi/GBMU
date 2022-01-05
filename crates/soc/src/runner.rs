use cpu::Cpu;
use memory::Memory;
use shared::{Finished, Output, Run};
use std::task::{Context, Poll};

enum Processor {
    Cpu,
}

pub struct Runner {
    pub memory: Memory,
    tasks: Tasks,
}

impl Runner {
    pub fn new(memory: Memory, state: memory::State) -> Self {
        let cpu = match state {
            memory::State::Bios => Cpu::new(memory.clone(), true),
            memory::State::Rom => Cpu::new(memory.clone(), false),
        };
        let tasks = Tasks::new(cpu);
        Self { memory, tasks }
    }

    pub fn run(&mut self) -> Vec<Finished> {
        let waker = shared::waker::create();
        let mut context = Context::from_waker(&waker);

        self.memory.borrow_mut().clock_tick();
        let cpu_status = self.tasks.run(Processor::Cpu, &mut context);
        vec![cpu_status]
    }

    pub fn cpu(&self) -> Cpu {
        self.tasks.cpu.clone()
    }
}

struct Tasks {
    cpu: Cpu,
    cpu_process: Output,
}

impl Tasks {
    pub fn new(cpu: Cpu) -> Self {
        let cpu_process = cpu.clone().run();
        Self {
            cpu,
            cpu_process,
        }
    }

    fn run(&mut self, processor: Processor, context: &mut Context) -> Finished {
        match processor {
            Processor::Cpu => match self.cpu_process.as_mut().poll(context) {
                Poll::Ready(status) => {
                    self.cpu_process = self.cpu.clone().run();
                    println!("Instruction Cpu Finie");
                    self.cpu.borrow_mut().print_debug();
                    Finished::finish(status)
                }
                Poll::Pending => {
                    self.cpu.borrow_mut().print_debug();
                    Finished::Nope
                }
            },
        }
    }
}
