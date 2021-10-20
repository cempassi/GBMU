use std::cell::RefCell;
use std::rc::Rc;

pub type Runner = Rc<RefCell<Mode>>;

#[derive(Debug)]
pub enum Mode {
    Tick(bool),
}

impl Mode {
    pub fn check(&mut self) -> bool {
        match self {
            Mode::Tick(ref mut status) if *status => {
                let current = *status;
                *status = !*status;
                current
            }
            _ => false,
        }
    }

    pub fn tick(&mut self) {
        *self = Mode::Tick(true);
    }
}

pub trait New {
    fn new() -> Self;
}

impl New for Runner {
    fn new() -> Self {
        Rc::new(RefCell::new(Mode::Tick(false)))
    }
}
