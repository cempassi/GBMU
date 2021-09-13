use super::consts;
use shared::{traits::Bus, Error};

pub struct Mbc0 {
    data: Vec<u8>,
}

impl Bus<usize> for Mbc0 {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
        self.data[address]
    }

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
        Err(Error::IllegalSet(address, data))
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
    use shared::traits::Bus;

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
