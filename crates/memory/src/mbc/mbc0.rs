use super::consts;
use shared::Error;
use super::bus::MbcBus;
use crate::MemoryBus;

#[derive(Debug)]
pub struct Mbc0 {
    data: Vec<u8>,
}

impl MbcBus for Mbc0 {
    fn set(&mut self, address: usize, data: u8) ->  Result<(), Error>{
        Err(Error::IllegalSet(address, data))
    }
}

impl MemoryBus for Mbc0 {
    fn get(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn set(&mut self, address: usize, data: Self::Data){
    }
}

impl Mbc0 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc0 { data }
    }
}

impl Default for Mbc0 {
    fn default() -> Self {
        Mbc0::new(vec![0; consts::MBC0_MAX_SIZE])
    }
}

#[cfg(test)]
mod test_nombc {
    use super::Mbc0;
    use super::MbcBus;

    #[test]
    fn test_read_nombc() {
        let mbc0 = Mbc0::default();

        assert_eq!(mbc0.get(0x10), 0);
    }

    #[test]
    fn test_write_read_nombc() {
        let mut mbc0 = Mbc0::default();

        match mbc0.set(0x42, 42) {
            Ok(_) => panic!(),
            Err(_) => (),
        }
    }
}
