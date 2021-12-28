use shared::Error;
use shared::Interrupt;
use shared::Interrupts as Registered;

/// enabled: Master interupt flag
/// registered: IE Interrupt enabled
/// requested: IE Interrupt flag

/// Any set bits in the IF register are only requesting an interrupt to be executed.
#[derive(Debug, Default)]
pub struct Interrupts {
    is_interrupted: bool,
    master_enabled: bool,
    enabled: Registered,
    requested: Registered,
}

impl Interrupts {
    pub fn set_is_interrupted(&mut self) {
        self.is_interrupted = true;
    }

    pub fn disabled_is_interrupted(&mut self) {
        self.is_interrupted = false;
    }

    pub fn disable_master_enabled(&mut self) {
        self.master_enabled = false;
    }

    pub fn set_master_enabled(&mut self) {
        self.master_enabled = true;
    }
    pub fn master_enabled(&self) -> bool {
        self.master_enabled
    }

    pub fn check(&mut self) {
        if self.is_interrupted {
            self.master_enabled = true;
        }
    }

    pub fn is_requested(&self) -> bool {
        self.requested.borrow().check(0xFF) != 0
    }

    pub fn get_raisable(&self) -> Registered {
        self.requested.clone()
    }

    pub fn set_enabled(&mut self, data: u8) -> Result<(), Error> {
        println!("[MEM] Interrupt enabled");
        self.enabled.borrow_mut().set(data)
    }

    pub fn get_enabled(&self) -> Result<u8, Error> {
        self.enabled.borrow().get()
    }

    pub fn set_requested(&mut self, data: u8) -> Result<(), Error> {
        println!("[MEM] Interrupt flag");
        self.requested.borrow_mut().set(data)
    }

    pub fn get_requested(&self) -> Result<u8, Error> {
        self.requested.borrow().get()
    }

    fn get_status(&self, interrupt: Interrupt) -> bool {
        self.requested.borrow().status(interrupt) && self.enabled.borrow().status(interrupt)
    }

    pub fn get_address(&mut self) -> Option<u16> {
        if self.get_status(Interrupt::VBlank) {
            self.requested.borrow_mut().processed(Interrupt::VBlank);
            Some(0x40)
        } else if self.get_status(Interrupt::Lcd) {
            self.requested.borrow_mut().processed(Interrupt::Lcd);
            Some(0x48)
        } else if self.get_status(Interrupt::Timer) {
            self.requested.borrow_mut().processed(Interrupt::Timer);
            Some(0x50)
        } else if self.get_status(Interrupt::Serial) {
            self.requested.borrow_mut().processed(Interrupt::Serial);
            Some(0x58)
        } else if self.get_status(Interrupt::Joypad) {
            self.requested.borrow_mut().processed(Interrupt::Joypad);
            Some(0x60)
        } else {
            None
        }
    }
}
