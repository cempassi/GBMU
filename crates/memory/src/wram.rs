use crate::MemoryBus;

const WRAM_SIZE: usize = 8192;

#[derive(Debug)]
pub struct Wram {
    data: [u8; WRAM_SIZE],
}

impl Default for Wram {
    fn default() -> Self {
        Self::new()
    }
}

impl Wram {
    pub fn new() -> Self {
        Wram {
            data: [0; WRAM_SIZE],
        }
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
