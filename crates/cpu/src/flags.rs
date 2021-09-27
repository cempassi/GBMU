use super::area::Flag;
use crate::RegisterBus;
use modular_bitfield::{bitfield, specifiers::B4};

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Flags {
    #[skip]
    unuzed: B4,
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Flags::new()
    }
}

impl RegisterBus<Flag, bool> for Flags {
    fn get(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.z(),
            Flag::N => self.n(),
            Flag::H => self.h(),
            Flag::C => self.c(),
        }
    }

    fn set(&mut self, flag: Flag, data: bool) {
        match flag {
            Flag::Z => self.set_z(data),
            Flag::N => self.set_n(data),
            Flag::H => self.set_h(data),
            Flag::C => self.set_c(data),
        }
    }
}

#[cfg(test)]
mod test_flags {
    use super::Flag;
    use super::Flags;
    use crate::RegisterBus;

    #[test]
    fn test_valid_flag_set_get() {
        let mut flags: Flags = Flags::new();

        flags.set(Flag::Z, true);
        let value = flags.get(Flag::Z);
        assert_eq!(value, true);
    }
}
