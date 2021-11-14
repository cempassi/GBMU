use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub type SOC = Rc<RefCell<crate::soc::SOC>>;

#[derive(Default, Debug, Clone)]
pub struct Status(Rc<RefCell<crate::status::Status>>);

impl Status {
    pub fn new(cpu: cpu::Cpu) -> Self {
        Self {
            0: Rc::new(RefCell::new(crate::status::Status::new(cpu))),
        }
    }
}

impl Deref for Status {
    type Target = Rc<RefCell<crate::status::Status>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Status {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait TryInit {
    fn try_init(rom: &str) -> Result<Self, std::io::Error>
    where
        Self: std::marker::Sized;
}

impl TryInit for SOC {
    fn try_init(rom: &str) -> Result<Self, std::io::Error> {
        let soc = crate::soc::SOC::try_from(rom)?;
        Ok(Rc::new(RefCell::new(soc)))
    }
}
