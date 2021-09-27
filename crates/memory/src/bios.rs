use std::fs;
use std::path::Path;
#[derive(Debug)]
pub struct Bios {
    data: Vec<u8>,
}

impl Default for Bios {
    fn default() -> Self {
        Self::new()
    }
}

impl Bios {
    pub fn new() -> Self {
        let path = Path::new("ressources/bios/dmg_boot.bin");
        let data = fs::read(path).unwrap();
        Bios { data }
    }

    pub fn set(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }

    pub fn get(&self, address: usize) -> u8 {
        self.data[address]
    }
}

#[cfg(test)]
mod test_bios {
    //use super::Bios;

    // #[test]
    // fn test_read_wram() {
    //     let wram = Bios::default();

    //     assert_eq!(wram.get(0x10), 0);
    // }

    // #[test]
    // fn test_write_read_wram() {
    //     let mut wram = Wram::default();

    //     wram.set(0x42, 42);
    //     let read = wram.get(0x42);

    //     assert_eq!(read, 42);
    // }
}
