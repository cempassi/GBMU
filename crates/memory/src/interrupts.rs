use shared::Interrupts as Registered;
/// enabled: Master interupt flag
/// registered: IE Interrupt enabled
/// requested: IE Interrupt flag

/// Any set bits in the IF register are only requesting an interrupt to be executed.
#[derive(Debug, Default)]
pub struct Interrupts {
    is_interrupted: bool,
    enabled: bool,
    registred: Registered,
    requested: Registered
}

impl Interrupts {
    pub fn enable(&mut self){
        if !self.is_interrupted  {
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

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn get_requested(&self) -> Registered {
        self.requested.clone()
    }
}
