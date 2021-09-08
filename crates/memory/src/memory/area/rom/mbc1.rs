use shared::{traits::Bus, Error};
// pub const MBC1_MAX_SIZE: usize = 16_384; // / 8;
// pub const BANK_SIZE: usize = 8 * 16 * 1024;

// #[derive(Default)]
pub struct Mbc1 {
	// ram_enable: bool,
	// bank_mode: bool, // 0 false = rom mode / 1 true = ram mode
	data: Vec<u8>,
	// ram: &[u8],
	// rom: &[u8],
	// rom_bank_0: &[u8; BANK_SIZE],
	// rom_bank_1: &[u8; BANK_SIZE],
	// ram_offset: usize,
	// rom_offset: usize,
}

// pub struct Mbc1CMD
// {
//
// }

impl Bus<usize> for Mbc1 {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
    	match address {
            0x0000 ..= 0x3fff => self.data[address],
            0x4000 ..= 0x7fff => self.data[address], // - 0x4000 + self.rom_offset],
            _ => panic!("Address out of range 0x{:x}", address),
        }
    }

    fn set(&mut self, _address: usize, _data: Self::Data) -> Self::Result {
		unimplemented!()
		// match address {
		// 	0x0000 .. 0x1FFF => update_ram_lock(data), // enable RAM REG0
		// 	0x2000 .. 0x5FFF => update_bank_nbr(adress, data), // change bank nbr REG1 REG2
		// 	0x6000 .. 0x7FFF => update_bank_mode(data), // ROM RAM change REG3
		// 	_ => unreachable!(),
		// 	Ok (self.update_rom_offset());
		// }
    }
}

impl Mbc1 {
	pub fn new(data: Vec<u8>) -> Self {
		Mbc1 {
			// ram_enable: false,
			// bank_mode: false,
			data,
			// ram,
			// rom,
			// rom_bank_0: data[0..0x3fff],
			// rom_bank_1: data[0x4000..0x7fff],
			// ram_offset: 0,
			// rom_offset: 0x4000,
		}
	}
//
// 	fn update_bank_mode(&mut self, data: u8) {
// 		self.bank_mode = match data {
// 			0 => false,
// 			1 => true,
// 		}
// 	}
//
// 	fn update_bank_nbr(&mut self, adress: usize, data: u8) {
// 		match address {
// 			0x2000 .. 0x3FFF => {
// 				if data < 0x1F { // 127
// 					let bk_nbr = (data & 0x1F);
// 					if (bk_nbr & 0x0F) = 0x00 { bk_nbr |= 0x0F }
// 					println!("bank nbr {:?}", bk_nbr);
// 				} else {
// 					0
// 				}
// 			},
// 			0x4000 .. 0x5FFF => {
// 				if !self.bank_mode {
// 					if data < 0x1F { // 127
// 						let bk_nbr = (data & 0x1F);
// 						if (bk_nbr & 0x0F) = 0x00 { bk_nbr |= 0x0F }
// 						println!("bank nbr {:?}", bk_nbr);
//
// 					} else {
// 						0
// 					}
// 				} else {
// 					implemented!()
// 				}
// 			},
// 			_ => unreachable!(),
// 		}
//
// 	}
//
// 	fn update_ram_lock(&mut self, data: u8) {
// 		if data = 0x0A {
// 			self.ram_enable = true
// 		} else {
// 			self.ram_enable = false
// 		}
// 	}
//
// 	fn update_ram_offset(&mut self) {
// 		self.ram_offset = if self.bank_mode == 1 {
// 			self.rom_bank_1 as usize * 8 * 1024
// 		} else {
// 			0
// 		}
// 	}
//
// 	fn update_rom_offset(&mut self) {
// 		let bank0: usize = match self.rom_bank_0 {
// 			0 => 1,
// 			_ => {
// 				match self.rom_bank_0 & 0xf0 {
// 					0x20 | 0x40 | 0x60 => self.rom_bank_0 | 0x01,
// 					_ => self.rom_bank_0,
// 				}
// 			}
// 		};
//
// 		let bank1: usize = match self.rom_bank_1 {
// 			0 => self.rom_bank_1 & 0b11,
// 			_ => 0,
// 		};
//
// 		self.rom_offset = bank0 * 0x4000 + bank1 * 512 * 1024;
// 	}
//
}

// impl Default for Mbc1 {
// 	fn default() -> Self {
// 		Mbc1::new(vec![0; MBC1_MAX_SIZE])
// 	}
// }


#[cfg(test)]
mod mbc1_test {

	use super::Mbc1;
	use std::fs;
	use shared::traits::Bus;

	// #[test]
	// fn test_mbc1_new() {
	// 	let rom_file = fs::read("Metroid II - Return of Samus.gb");
	// 	let mbc = Mbc1::new(rom_file);
	//
	// 	assert_eq!()
	// }

	#[test]
	fn test_mbc1_get()  {
		let rom_file = fs::read("Metroid.gb").unwrap();
		// let rom_file = fs::read("roms/Metroid.gb").unwrap();
		let mbc = Mbc1::new(rom_file);
		let data = mbc.get(0x01c0);

		assert_eq!(data, 0xbb);
	}

}