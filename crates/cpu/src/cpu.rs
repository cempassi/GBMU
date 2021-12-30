use crate::registers::Registers;
use memory::Memory;

#[derive(Default, Debug)]
pub struct Cpu {
    memory: Memory,
    pub registers: Registers,
    pub(crate) halt: bool,
    pub(crate) stop: bool,
}

impl Cpu {
    pub fn new(memory: Memory, bios: bool) -> Self {
        let registers = match bios {
            true => Registers::default(),
            false => Registers::new(),
        };
        Self {
            memory,
            registers,
            halt: false,
            stop: false,
        }
    }

    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn master_enabled(&self) -> bool {
        self.memory.borrow().master_enabled()
    }

    pub fn print_debug(&mut self) {
        if self.memory.borrow_mut().get_debug().is_some() {}
    }
}
