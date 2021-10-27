use crate::MemoryBus;
use std::convert::AsRef;

const WRAM_SIZE: usize = 8192;

#[derive(Debug)]
pub struct Ram {
    data: Vec<u8>,
}

impl Default for Ram {
    fn default() -> Self {
        Self::new(WRAM_SIZE)
    }
}

impl AsRef<Vec<u8>> for Ram {
    fn as_ref(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}

impl MemoryBus for Ram {
    fn get(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn set(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Ram {
            data: vec![0; size],
        }
    }
}

#[cfg(test)]
mod test_wram {
    use super::Ram;
    use crate::MemoryBus;

    #[test]
    fn test_read_wram() {
        let wram = Ram::default();

        assert_eq!(wram.get(0x10), 0);
    }

    #[test]
    fn test_write_read_wram() {
        let mut wram = Ram::default();

        wram.set(0x42, 42);
        let read = wram.get(0x42);

        assert_eq!(read, 42);
    }
}
