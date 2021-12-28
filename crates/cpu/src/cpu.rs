use crate::registers::Registers;
use memory::Memory;

#[derive(Default, Debug)]
pub struct Cpu {
    memory: Memory,
    pub registers: Registers,
    pub(crate) halt: bool,
    pub(crate) stop: bool,
    debug: String,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            debug: String::new(),
            registers: Registers::default(),
            halt: false,
            stop: false,
        }
    }

    pub fn no_bios(memory: Memory) -> Self {
        Self {
            memory,
            debug: String::new(),
            registers: Registers::no_bios(),
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
        if let Some(character) = self.memory.borrow_mut().get_debug() {
            self.debug.push(character);
            println!("[DBG] {}", self.debug);
        }
    }
}
