use shared::Error;
use crate::MemoryBus;

pub(crate) trait MbcBus: MemoryBus {
    fn set(&mut self, address: usize, data: u8) ->  Result<(), Error>;
}
