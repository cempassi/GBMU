use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPC(u16),
    IllegalSet(usize, u8),
    InvalidGet(u16),
    InvalidSet(u16, u8),
    RamLock(usize),
    Unimplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidPC(pc) => write!(f, "Invalid PC: {:#08X}", pc),
            Error::IllegalSet(address, data) => write!(
                f,
                "Illegal Set. Address: {:#08X}, data: {:04X}",
                address, data
            ),
            Error::InvalidGet(address) => write!(f, "Invalid Get. Address: {:#08X}", address),
            Error::InvalidSet(address, data) => write!(
                f,
                "Invalid Set. Address: {:#08X}, data: {:04X}",
                address, data
            ),
            Error::RamLock(data) => write!(f, "Ram Lock: data: {:#x}", data),
            Error::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}
