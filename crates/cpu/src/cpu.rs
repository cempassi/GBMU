pub use crate::interface::Registers;
use memory::Memory;

#[derive(Default, Clone)]
pub struct Cpu {
    memory: Memory,
    registers: Registers,
    pub(crate) halt: bool,
    pub(crate) stop: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: Registers::default(),
            halt: false,
            stop: false,
        }
    }

    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn get_registers(&self) -> Registers {
        self.registers.clone()
    }

    pub fn interrupt_enabled(&self) -> bool {
        self.memory.borrow().is_enabled().is_ok()
    }
}
