use std::fs;
use std::path::PathBuf;
use std::str;
use crate::MemoryBus;

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
        let output = std::process::Command::new("git")
            .args(&["rev-parse", "--show-toplevel"])
            .output()
            .unwrap();
        let git_root = str::from_utf8(&output.stdout).unwrap().trim();
        let mut path = PathBuf::new();
        path.push(git_root);
        path.push("ressources/bios/dmg_boot.bin");
        println!("path: {:?}", path);
        let data = fs::read(path).unwrap();
        Bios { data }
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
