use super::bus::Mbc;
use super::consts;
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

impl Mbc for Mbc0 {
    fn get_rom(&self, address: usize) -> Result<u8, Error> {
        match address as u16 {
            crate::consts::ROM_MIN..=crate::consts::ROM_MAX => Ok(self.data[address]),
            _ => Ok(0xFF),
        }
    }

    fn set_rom(&mut self, _address: usize, _data: u8) -> Result<(), Error> {
        Ok(())
    }

    fn get_ram(&self, _: usize) -> Result<u8, Error> {
        Ok(0)
    }

    fn set_ram(&mut self, _: usize, _: u8) -> Result<(), Error> {
        Ok(())
    }
}

impl Mbc0 {
    pub fn new(data: Vec<u8>) -> Box<Self> {
        Box::new(Self { data })
    }
}

#[cfg(test)]
mod test_nombc {
    use super::Mbc;
    use super::Mbc0;

    #[test]
    fn test_read_nombc() {
        let mbc0 = Mbc0::default();

        assert_eq!(mbc0.get_rom(0x10).unwrap(), 0);
    }
}
