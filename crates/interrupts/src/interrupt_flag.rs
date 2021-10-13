/// 0xFF0F - IF - Interrupt Flag (R/W)
///  Bit 0: VBlank   Interrupt Request (INT $40)  (1=Request)
///  Bit 1: LCD STAT Interrupt Request (INT $48)  (1=Request)
///  Bit 2: Timer    Interrupt Request (INT $50)  (1=Request)
///  Bit 3: Serial   Interrupt Request (INT $58)  (1=Request)
///  Bit 4: Joypad   Interrupt Request (INT $60)  (1=Request)
pub struct InterruptsFlag {
    enable: Rc<RefCell<Interrupts>>,
    request: Rc<RefCell<Interrupts>>,
}

impl InterruptsFlag {
    pub fn new() -> InterruptsFlag {
        InterruptsFlag {
            enable: Rc::new(RefCell::new(Interrupts::default())),
            request: Rc::new(RefCell::new(Interrupts::default())),
        }
    }

    pub fn interrupt_request(&self) -> InterruptsRequest {
        InterruptsRequest::new(self.request.clone())
    }

    pub fn peek(&self) -> Option<u8> {
        self.check(false)
    }

    pub fn poll(&self) -> Option<u8> {
        self.check(true)
    }

    fn check(&self, consume: bool) -> Option<u8> {
        let e = self.enable.borrow();
        let mut r = self.request.borrow_mut();

        if e.vblank && r.vblank {
            r.vblank = !consume;
            Some(0x40)
        } else if e.lcd && r.lcd {
            r.lcd = !consume;
            Some(0x48)
        } else if e.timer && r.timer {
            r.timer = !consume;
            Some(0x50)
        } else if e.serial && r.serial {
            r.serial = !consume;
            Some(0x58)
        } else if e.joypad && r.joypad {
            r.joypad = !consume;
            Some(0x60)
        } else {
            None
        }
    }
}