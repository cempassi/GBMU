use super::memory::Memory;
use shared::Error;
use shared::Interrupts as Registered;

/// enabled: Master interupt flag
/// registered: IE Interrupt enabled
/// requested: IE Interrupt flag

/// Any set bits in the IF register are only requesting an interrupt to be executed.
#[derive(Debug, Default)]
pub struct Interrupts {
    is_interrupted: bool,
    enabled: bool,
    pub(crate) registred: Registered,
    pub(crate) requested: Registered,
}

impl Interrupts {
    pub fn enable(&mut self) {
        self.is_interrupted = true;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn check(&mut self) {
        if self.is_interrupted {
            self.enabled = true;
            self.is_interrupted = false;
        }
    }

    pub fn is_requested(&self) -> bool {
        self.requested.borrow().check(0xFF) != 0
    }

    pub fn requested(&self) -> u8 {
        let requested = self.requested.borrow().get().unwrap();

        self.registred.borrow().check(requested)
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn get_raisable(&self) -> Registered {
        self.requested.clone()
    }

    pub fn get_address(&mut self) -> Option<u16> {
        let registered = self.registred.borrow_mut();
        let mut requested = self.requested.borrow_mut();

        if registered.vblank() && requested.vblank() {
            requested.set_vblank(false);
            Some(0x40)
        } else if registered.lcd() && requested.lcd() {
            requested.set_lcd(false);
            Some(0x48)
        } else if registered.timer() && requested.timer() {
            requested.set_timer(false);
            Some(0x50)
        } else if registered.serial() && requested.serial() {
            requested.set_serial(false);
            Some(0x58)
        } else if registered.joypad() && requested.joypad() {
            requested.set_joypad(false);
            Some(0x60)
        } else {
            None
        }
    }
}

impl Memory {
    pub fn get_interrupt_address(&mut self) -> Option<u16> {
        self.interrupts.get_address()

    }

    pub fn enable_interrupts(&mut self) -> u8 {
        self.interrupts.enable();
        0
    }

    pub fn get_requested(&self) -> Result<u32, Error> {
        let requested = self.interrupts.requested();
        if requested != 0 {
            Ok(requested.trailing_zeros())
        } else {
            Err(Error::DisabledInterrupts)
        }
    }

    pub fn is_requested(&self) -> bool {
        self.interrupts.is_requested()
    }

    pub fn disable_interrupts(&mut self) -> u8 {
        self.interrupts.disable();
        0
    }

    pub fn is_enabled(&self) -> Result<(), Error> {
        if self.interrupts.is_enabled() {
            Ok(())
        } else {
            Err(Error::DisabledInterrupts)
        }
    }

    /// Check if EI instruction was called, set interrupt if it was
    pub fn check_interrupts(&mut self) {
        self.interrupts.check()
    }
}
