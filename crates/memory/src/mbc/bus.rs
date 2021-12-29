use shared::Error;

pub trait Mbc: std::fmt::Debug + AsRef<Vec<u8>> {
    fn get_rom(&self, _: usize) -> Result<u8, Error>;
    fn set_rom(&mut self, address: usize, data: u8) -> Result<(), Error>;
    fn get_ram(&self, _: usize) -> Result<u8, Error>;
    fn set_ram(&mut self, address: usize, data: u8) -> Result<(), Error>;
}
