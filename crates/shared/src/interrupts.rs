//use modular_bitfield::{bitfield, specifiers::B3};

pub mod interface {
    use std::cell::RefCell;
    use std::rc::Rc;
    pub type Interrupts = Rc<RefCell<super::Interrupts>>;
}

//#[bitfield]
#[derive(Debug, Default)]
pub struct Interrupts {
    //pub(crate) vblank: bool,
    //pub(crate) lcd: bool,
    //pub(crate) timer: bool,
    //pub(crate) serial: bool,
    //pub(crate) joypad: bool,
    //#[skip]
    _unused: bool,
}

// impl Interrupts {
//     pub fn processed(&mut self, interrupt: Interrupt) {
//         match interrupt {
//             Interrupt::Vblank => self.set_vblank(false),
//             Interrupt::Lcd => self.set_lcd(false),
//             Interrupt::Timer => self.set_timer(false),
//             Interrupt::Serial => self.set_serial(false),
//             Interrupt::Joypad =>self.set_joypad(false)
//         }
//     }
//
//     pub fn request(&mut self, interrupt: Interrupt) {
//         match interrupt {
//             Interrupt::Vblank => self.set_vblank(true),
//             Interrupt::Lcd => self.set_lcd(true),
//             Interrupt::Timer => self.set_timer(true),
//             Interrupt::Serial => self.set_serial(true),
//             Interrupt::Joypad =>self.set_joypad(true)
//         }
//     }
// }
//
// pub enum Interrupt {
//     Vblank,
//     Lcd,
//     Timer,
//     Serial,
//     Joypad
// }
//
//
