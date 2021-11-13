use shared::Error;

pub trait MbcBus: std::fmt::Debug + AsRef<Vec<u8>> {
    fn get(&self, _: usize) -> Result<u8, Error>;
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error>;
}
