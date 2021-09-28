use crate::MemoryBus;
use shared::Error;

pub(crate) trait MbcBus: MemoryBus {
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error>;
}
