use std::cell::RefCell;
use std::rc::Rc;

pub type SOC = Rc<RefCell<crate::soc::SOC>>;

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
