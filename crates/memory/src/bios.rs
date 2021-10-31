use crate::MemoryBus;
use shared::Error;
use std::convert::AsRef;
use std::fs;
use std::path::PathBuf;
use std::str;

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
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if let Some(index) = self.data.get_mut(address) {
            *index = data;
            Ok(())
        } else {
            Err(Error::InvalidSet(address, data))
        }
    }

    fn get(&self, address: usize) -> Result<u8, Error> {
        if let Some(index) = self.data.get(address) {
            Ok(*index)
        } else {
            Ok(0)
        }
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
