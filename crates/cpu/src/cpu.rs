pub use crate::interface::Registers;
use memory::Memory;

#[derive(Default, Clone)]
pub struct Cpu {
    memory: Memory,
    registers: Registers,
    halt: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            registers: Registers::default(),
            halt: false,
        }
    }

    pub fn set_halt(&mut self) -> u8 {
        self.halt = true;
        0
    }
    pub fn unset_halt(&mut self) {
        self.halt = false;
    }

    pub fn is_halted(&self) -> bool {
        self.halt
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
