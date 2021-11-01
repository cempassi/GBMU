use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPC(u16),
    IllegalSet(usize, u8),
    InvalidGet(usize),
    InvalidSet(usize, u8),
    InvalidInterupt(u32),
    DisabledInterrupts,
    FifoNotReady,
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
            Error::InvalidInterupt(interrupt) => {
                write!(f, "Invalid Interrupt. value: {}", interrupt)
            }
            Error::DisabledInterrupts => write!(f, "Disabled interrupts"),
            Error::FifoNotReady => write!(f, "Fifo Not Ready"),
        }
    }
}
