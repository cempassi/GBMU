use super::bus::Mbc;
use crate::{Cartridge, Header};
use shared::Error;
use std::convert::AsRef;
use std::io::prelude::*;
use std::{fs, io, path};

#[derive(Debug)]
pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    ram_mode: bool,
    rombank: usize,
    rambank: usize,
    savepath: Option<path::PathBuf>,
}

impl AsRef<Vec<u8>> for Mbc1 {
    fn as_ref(&self) -> &Vec<u8> {
        self.rom.as_ref()
    }
}

impl Drop for Mbc1 {
    fn drop(&mut self) {
        match self.savepath {
            None => {}
            Some(ref path) => {
                let _ = fs::File::create(path).and_then(|mut f| f.write_all(&*self.ram));
            }
        };
    }
}

impl Mbc for Mbc1 {
    fn get_rom(&self, address: usize) -> Result<u8, Error> {
        let index = if address < 0x4000 {
            address as usize
        } else {
            (self.rombank * 0x4000) | ((address as usize) & 0x3FFF)
        };
        Ok(*self.rom.get(index).unwrap_or(&0))
    }

    fn set_rom(&mut self, address: usize, data: u8) -> Result<(), Error> {
        match address {
            0x0000..=0x1FFF => {
                self.ram_on = data == 0x0A;
            }
            0x2000..=0x3FFF => {
                self.rombank = (self.rombank & 0x60)
                    | match (data as usize) & 0x1F {
                        0 => 1,
                        n => n,
                    }
            }
            0x4000..=0x5FFF => {
                if !self.ram_mode {
                    self.rombank = self.rombank & 0x1F | (((data as usize) & 0x03) << 5)
                } else {
                    self.rambank = (data as usize) & 0x03;
                }
            }
            0x6000..=0x7FFF => {
                self.ram_mode = (data & 0x01) == 0x01;
            }
            _ => panic!("Could not write to {:04X} (MBC1)", data),
        };
        Ok(())
    }

    fn get_ram(&self, address: usize) -> Result<u8, Error> {
        if !self.ram_on {
            return Ok(0);
        }
        let rambank = if self.ram_mode { self.rambank } else { 0 };
        Ok(self.ram[(rambank * 0x2000) | ((address & 0x1FFF) as usize)])
    }

    fn set_ram(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if !self.ram_on {
            return Ok(());
        }
        let rambank = if self.ram_mode { self.rambank } else { 0 };
        self.ram[(rambank * 0x2000) | ((address & 0x1FFF) as usize)] = data;
        Ok(())
    }
}

impl Mbc1 {
    pub fn new(header: Header, data: Vec<u8>, file: path::PathBuf) -> Box<Self> {
        let (svpath, ramsize) = match header.cartridge {
            Cartridge::Mbc1Ram => (None, header.ram_size.get_size()),
            Cartridge::Mbc1RamBattery => (
                Some(file.with_extension("gbsave")),
                header.ram_size.get_size(),
            ),
            _ => (None, 0),
        };

        let mut res = Mbc1 {
            rom: data,
            ram: ::std::iter::repeat(0u8).take(ramsize).collect(),
            ram_on: false,
            ram_mode: false,
            rombank: 1,
            rambank: 0,
            savepath: svpath,
        };
        let _ = res.loadram();
        Box::new(res)
    }

    fn loadram(&mut self) -> Result<(), Error> {
        match self.savepath {
            None => Ok(()),
            Some(ref savepath) => {
                let mut data = vec![];
                match fs::File::open(savepath).and_then(|mut f| f.read_to_end(&mut data)) {
                    Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
                    Err(_) => Err(Error::FailedRamLoad),
                    Ok(..) => {
                        self.ram = data;
                        Ok(())
                    }
                }
            }
        }
    }
}
