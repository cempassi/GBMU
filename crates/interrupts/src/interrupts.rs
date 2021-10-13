use alloc::rc::Rc;
use core::cell::RefCell;
use shared::Error;
use consts::{INTERRUPT_ENABLE, INTERRUPT_FLAG};



struct Interrupts {
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Interrupts {
    fn set(&mut self, value: u8) {
        self.vblank = value & 0x01 != 0;
        self.lcd = value & 0x02 != 0;
        self.timer = value & 0x04 != 0;
        self.serial = value & 0x08 != 0;
        self.joypad = value & 0x10 != 0;
    }

    fn get(&self) -> u8 {
        let mut v = 0;
        v |= if self.vblank { 0x01 } else { 0x00 };
        v |= if self.lcd { 0x02 } else { 0x00 };
        v |= if self.timer { 0x04 } else { 0x00 };
        v |= if self.serial { 0x08 } else { 0x00 };
        v |= if self.joypad { 0x10 } else { 0x00 };
        v
    }
}



impl IoHandler for InterruptsFlag {
    fn on_read(&mut self, address: u16) -> Result<u8, Error>{
        if address == INTERRUPT_ENABLE {
            Ok(self.enable.borrow().get())
        } else if address == INTERRUPT_FLAG {
            Ok(self.request.borrow().get())
        } else {
            Err(Error::SegmentationFault(address))
        }
    }

    fn on_write(&mut self, memory: Memory, addr: u16, value: u8) -> MemWrite {
        if addr == INTERRUPT_ENABLE {
            info!("Write interrupt enable: {:02x}", value);
            self.enable.borrow_mut().set(value);
            MemWrite::Block
        } else if addr == INTERRUPT_FLAG {
            info!("Write interrupt: {:02x}", value);
            self.request.borrow_mut().set(value);
            MemWrite::Block
        } else {
            info!("Writing to IC register: {:04x}", addr);
            MemWrite::PassThrough
        }
    }
}
