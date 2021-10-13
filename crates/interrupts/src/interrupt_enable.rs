pub struct InterruptsRequest {
    request: Rc<RefCell<Interrupts>>,
}

/// 0xFFFF - IE - Interrupt Enable (R/W)
///  Bit 0: VBlank   Interrupt Enable  (INT $40)  (1=Enable)
///  Bit 1: LCD STAT Interrupt Enable  (INT $48)  (1=Enable)
///  Bit 2: Timer    Interrupt Enable  (INT $50)  (1=Enable)
///  Bit 3: Serial   Interrupt Enable  (INT $58)  (1=Enable)
///  Bit 4: Joypad   Interrupt Enable  (INT $60)  (1=Enable)
impl InterruptsRequest {
    fn new(request: Rc<RefCell<Interrupts>>) -> InterruptsRequest {
        InterruptsRequest { request }
    }

    pub fn vblank(&self, v: bool) {
        self.request.borrow_mut().vblank = v;
    }

    pub fn lcd(&self, v: bool) {
        self.request.borrow_mut().lcd = v;
    }

    pub fn timer(&self, v: bool) {
        self.request.borrow_mut().timer = v;
    }

    pub fn serial(&self, v: bool) {
        self.request.borrow_mut().serial = v;
    }

    pub fn joypad(&self, v: bool) {
        self.request.borrow_mut().joypad = v;
    }
}