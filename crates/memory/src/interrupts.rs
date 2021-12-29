use shared::Error;
use shared::Interrupt;
use shared::Interrupts as Registered;

/// enabled: Master interupt flag
/// registered: IE Interrupt enabled
/// requested: IE Interrupt flag

/// Any set bits in the IF register are only requesting an interrupt to be executed.
#[derive(Debug, Default)]
pub struct Interrupts {
    is_interrupted: u8,
    is_disabled: u8,
    master_enabled: bool,
    enabled: Registered,
    requested: Registered,
}

impl Interrupts {
    // is_interrupted functions
    pub fn set_is_interrupted(&mut self, delay: u8) {
        self.is_interrupted = delay;
    }

    pub fn disabled_is_interrupted(&mut self) {
        self.is_interrupted = 0;
    }

    pub fn is_interrupted_control(&mut self) {
        self.is_interrupted = match self.is_interrupted {
            2 => 1,
            1 => {
                self.master_enabled = true;
                0
            }
            _ => 0,
        };
    }

    // is_dissabled functions
    pub fn set_is_dissabled(&mut self) {
        self.is_disabled = 2;
    }

    pub fn is_disabled_control(&mut self) {
        self.is_disabled = match self.is_disabled {
            2 => 1,
            1 => {
                self.master_enabled = false;
                0
            }
            _ => 0,
        };
    }

    // master_enabled functions
    pub fn master_enabled(&self) -> bool {
        self.master_enabled
    }

    pub fn set_master_enabled(&mut self) {
        self.master_enabled = true;
    }

    pub fn disable_master_enabled(&mut self) {
        self.master_enabled = false;
    }

    pub fn is_requested(&self) -> bool {
        self.requested.borrow().check(0xFF) != 0
    }

    pub fn is_triggered(&self) -> bool {
        let enabled = self.get_enabled().unwrap();
        let requested = self.get_requested().unwrap();

        enabled & requested != 0
    }

    pub fn get_raisable(&self) -> Registered {
        self.requested.clone()
    }

    pub fn set_enabled(&mut self, data: u8) -> Result<(), Error> {
        self.enabled.borrow_mut().set(data)
    }

    pub fn get_enabled(&self) -> Result<u8, Error> {
        self.enabled.borrow().get()
    }

    pub fn set_requested(&mut self, data: u8) -> Result<(), Error> {
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

#[cfg(test)]
mod test_interrupts {
    use super::*;

    #[test]
    fn test_interrupt_enabling() {
        let mut interrupts = Interrupts::default();
        assert!(!interrupts.master_enabled());

        interrupts.set_is_interrupted(2);
        assert!(!interrupts.master_enabled());

        interrupts.is_interrupted_control();
        assert!(!interrupts.master_enabled());

        interrupts.is_interrupted_control();
        assert!(interrupts.master_enabled());

        interrupts.disabled_is_interrupted();
        assert!(interrupts.master_enabled());

        interrupts.disable_master_enabled();
        assert!(!interrupts.master_enabled());
    }

    #[test]
    fn test_raise_lcd_interrupt() {
        let interrupts = Interrupts::default();
        let requested = interrupts.get_raisable();

        requested.borrow_mut().request(Interrupt::Lcd);

        let expected = 0b0000_0010;
        let result = interrupts.get_requested().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_raise_lcd_timer_serial_interrupt() {
        let interrupts = Interrupts::default();
        let requested = interrupts.get_raisable();

        requested.borrow_mut().request(Interrupt::Lcd);
        requested.borrow_mut().request(Interrupt::Timer);
        requested.borrow_mut().request(Interrupt::Serial);

        let expected = 0b0000_1110;
        let result = interrupts.get_requested().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_requested() {
        let interrupts = Interrupts::default();
        let requested = interrupts.get_raisable();

        requested.borrow_mut().request(Interrupt::Lcd);

        assert!(interrupts.is_requested());
    }

    #[test]
    fn test_lcd_enabled() {
        let mut interrupts = Interrupts::default();
        let requested = interrupts.get_raisable();

        let _ = interrupts.set_enabled(0b0000_0010);
        requested.borrow_mut().request(Interrupt::Lcd);

        assert!(interrupts.get_status(Interrupt::Lcd));
    }

    #[test]
    fn test_serial_disabled() {
        let mut interrupts = Interrupts::default();
        let requested = interrupts.get_raisable();

        let _ = interrupts.set_enabled(0b0000_0010);
        requested.borrow_mut().request(Interrupt::Lcd);

        assert!(!interrupts.get_status(Interrupt::Serial));
    }

    #[test]
    fn test_get_lcd_address() {
        let mut interrupts = Interrupts::default();
        let requested = interrupts.get_raisable();

        let _ = interrupts.set_enabled(0b0000_0010);
        requested.borrow_mut().request(Interrupt::Lcd);
        let address = interrupts.get_address();

        assert!(address.is_some());
        let address = address.unwrap();

        assert!(!interrupts.get_status(Interrupt::Lcd));
        assert_eq!(address, 0x48);
        assert_eq!(interrupts.get_requested().unwrap(), 0b0000_0000);
    }

    #[test]
    fn test_get_vblank_then_lcd_address() {
        let mut interrupts = Interrupts::default();
        let requested = interrupts.get_raisable();

        let _ = interrupts.set_enabled(0b0000_0011);
        requested.borrow_mut().request(Interrupt::VBlank);
        requested.borrow_mut().request(Interrupt::Lcd);

        // Get Vblank address
        let address = interrupts.get_address();

        assert!(address.is_some());
        let address = address.unwrap();

        assert!(!interrupts.get_status(Interrupt::VBlank));
        assert!(interrupts.get_status(Interrupt::Lcd));
        assert_eq!(address, 0x40);
        assert_eq!(interrupts.get_requested().unwrap(), 0b0000_0010);

        // Get LCD address
        let address = interrupts.get_address();

        assert!(address.is_some());
        let address = address.unwrap();

        assert!(!interrupts.get_status(Interrupt::Lcd));
        assert_eq!(address, 0x48);
        assert_eq!(interrupts.get_requested().unwrap(), 0b0000_0000);

        // Get None
        let address = interrupts.get_address();

        assert!(address.is_none());

        assert_eq!(interrupts.get_enabled().unwrap(), 0b0000_0011);
    }
}
