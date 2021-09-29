use crate::MemoryBus;
use std::convert::AsRef;

const WRAM_SIZE: usize = 8192;

#[derive(Debug)]
pub struct Wram {
    data: Vec<u8>,
}

impl Default for Wram {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<Vec<u8>> for Wram {
    fn as_ref(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}

impl MemoryBus for Wram {
    fn get(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn set(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }
}

impl Wram {
    pub fn new() -> Self {
        Wram {
            data: vec![0; WRAM_SIZE],
        }
    }
}

#[cfg(test)]
mod test_wram {
    use super::Wram;
    use crate::MemoryBus;

    #[test]
    fn test_read_wram() {
        let wram = Wram::default();

        assert_eq!(wram.get(0x10), 0);
    }

    #[test]
    fn test_write_read_wram() {
        let mut wram = Wram::default();

        wram.set(0x42, 42);
        let read = wram.get(0x42);

        assert_eq!(read, 42);
    }
}
