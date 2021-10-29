use super::memory::Memory;
use num_enum::TryFromPrimitive;
use shared::interrupts::Interrupt;
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
        if !self.is_interrupted {
            self.is_interrupted = true;
        } else {
            self.is_interrupted = false;
            self.enabled = true;
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn check(&mut self) {
        if self.is_interrupted {
            self.enable()
        }
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

    pub fn processed(&mut self, interrupt: Interrupt) {
        self.requested.borrow_mut().processed(interrupt);
    }
}

impl Memory {
    pub fn get_interrupt_address(&mut self, requested: u32) -> Result<u16, Error> {
        if let Ok(interrupt) = Interrupt::try_from_primitive(requested) {
            self.interrupts.processed(interrupt);
            let address = 0x0040 | ((requested as u16) << 3);
            Ok(address as u16)
        } else {
            Err(Error::InvalidInterupt(requested))
        }
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
