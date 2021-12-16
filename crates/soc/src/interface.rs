use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub type SOC = Rc<RefCell<crate::soc::SOC>>;

#[derive(Default, Debug, Clone)]
pub struct System(Rc<RefCell<crate::system::System>>);

impl System {
    pub fn new(cpu: cpu::Cpu) -> Self {
        Self {
            0: Rc::new(RefCell::new(crate::system::System::new(cpu))),
        }
    }
}

impl Deref for System {
    type Target = Rc<RefCell<crate::system::System>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for System {
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
