use crate::MemoryBus;
use std::fs;
use std::path::PathBuf;
use std::str;
use std::convert::AsRef;

#[derive(Debug)]
pub struct Bios {
    data: Vec<u8>,
}

impl Default for Bios {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<Vec<u8>> for Bios {
    fn as_ref(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}

impl MemoryBus for Bios {
    fn set(&mut self, address: usize, data: u8) {
        if let Some(index) = self.data.get_mut(address) {
            *index = data;
        }
    }

    fn get(&self, address: usize) -> u8 {
        if let Some(index) = self.data.get(address) {
            *index
        } else {
            0
        }
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
