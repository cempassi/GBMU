use super::Mbc0;
use crate::Rom;
use std::cell::RefCell;
use std::rc::Rc;

pub trait RomDefault {
    fn default() -> Self;
}

impl RomDefault for Rom {
    fn default() -> Self {
        Rc::new(RefCell::new(Mbc0::new(
            vec![0; crate::mbc::consts::MBC0_MAX_SIZE],
        )))
    }
}
