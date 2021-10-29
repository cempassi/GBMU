use super::bus::MbcBus;
use super::consts;
use super::Mbc;
use crate::MemoryBus;
use shared::Error;
use std::convert::AsRef;

#[derive(Debug)]
pub struct Mbc0 {
    data: Vec<u8>,
}

impl Default for Mbc0 {
    fn default() -> Self {
        Mbc0 {
            data: vec![0; consts::MBC0_MAX_SIZE],
        }
    }
}

impl AsRef<Vec<u8>> for Mbc0 {
    fn as_ref(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}

impl MbcBus for Mbc0 {
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        Err(Error::IllegalSet(address, data))
    }
}

impl MemoryBus for Mbc0 {
    fn get(&self, address: usize) -> Result<u8, Error> {
        Ok(self.data[address])
    }

    fn set(&mut self, _address: usize, _data: u8) -> Result<(), Error> {
        Ok(())
    }
}

impl Mbc for Mbc0 {}

impl Mbc0 {
    pub fn new(data: Vec<u8>) -> Box<Self> {
        Box::new(Self { data })
    }
}

#[cfg(test)]
mod test_nombc {
    use super::Mbc0;
    use super::MbcBus;
    use super::MemoryBus;

    #[test]
    fn test_read_nombc() {
        let mbc0 = Mbc0::default();

        assert_eq!(mbc0.get(0x10).unwrap(), 0);
    }

    #[test]
    fn test_write_read_nombc() {
        let mut mbc0 = Mbc0::default();

        assert!(<Mbc0 as MbcBus>::set(&mut mbc0, 0x42, 42).is_err());
    }
}
