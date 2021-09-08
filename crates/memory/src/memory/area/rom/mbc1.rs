use shared::{traits::Bus, Error};
// pub const MBC1_MAX_SIZE: usize = 16_384; // / 8;
// pub const BANK_SIZE: usize = 8 * 16 * 1024;

pub struct Mbc1 {
	ram_lock: bool,
	bank_mode: bool, // 0 false = rom mode / 1 true = ram mode
	data: Vec<u8>,
	rom_bank: u8,
	ram_bank: u8,
}

impl Bus<usize> for Mbc1 {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
		match address {
			0x0000 ..= 0x3fff => self.data[address],
			0x4000 ..= 0x7fff => {
				let bank_nbr = if self.rom_bank == 0{ 1 } else { self.rom_bank as usize }; //generic //flag usize usize et 2 offset
				let index = (bank_nbr as usize * 0x4000) | (address & 0x3FFF);
				self.data[index]
			},
			0xa000 ..= 0xbfff => {
				if !self.ram_lock { return 0 }
				let bank_nbr = if self.bank_mode { self.ram_bank as usize} else { 0 }; //generic
				let index = (bank_nbr as usize * 0x2000) | (address & 0x1FFF);
				self.data[index as usize]
			},
			_ => panic!("Invalid Address {:04X}", address),
		}
    }

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
		match address {
			0x0000 ..= 0x1FFF => Ok(Mbc1::update_ram_lock(self, data)), // enable RAM REG0
			0x2000 ..= 0x5FFF => Ok(Mbc1::update_bank_nbr(self, address, data)), // change bank nbr REG1 REG2
			0x6000 ..= 0x7FFF => Ok(Mbc1::update_bank_mode(self, data)), // change RAM bank nbr if  REG3
			0xa000 ..= 0xbfff => {
				if !self.ram_lock { return Err(shared::Error::IllegalSet(address, data)); } //fct write ram
				let bank_nbr = if self.bank_mode { self.ram_bank as usize } else { 0 };
				let index = (bank_nbr * 0x2000) | (address & 0x1FFF);
				self.data[index] = data;
				Ok(())
			}
			_ => Err(shared::Error::IllegalSet(address, data)),
		}
    }
}

impl Mbc1 {
	pub fn new(data: Vec<u8>) -> Self {
		Mbc1 {
			ram_lock: false,
			bank_mode: false,
			data,
			rom_bank: 0,
			ram_bank: 0,
		}
	}

	fn update_bank_mode(&mut self, data: u8) {
		self.bank_mode = match data & 0x01 {
			0 => false,
			1 => true,
			_ => unreachable!(),
		}
	}

	fn update_bank_nbr(&mut self, address: usize, data: u8) {
		match address {
			0x2000 ..= 0x3FFF => {
				let bk_nbr = data & 0x1F ;
				self.rom_bank |= match bk_nbr {
					0 => 1,
					20 => 21,
					40 => 41,
					60 => 61,
					_ => bk_nbr,
				}
			},
			0x4000 ..= 0x5FFF => {
				let bk_nbr = data & 0x03;
				if !self.bank_mode { // upper bit of rom bank nbr
					self.rom_bank |= bk_nbr << 5
				} else { // RAM bank nbr 00-11
					self.ram_bank = bk_nbr
				}
			},
			_ => unreachable!(),
		}

	}

	fn update_ram_lock(&mut self, data: u8) {
		if data == 0x0A {
			self.ram_lock = true
		} else {
			self.ram_lock = false
		}
	}
}

#[cfg(test)]
mod mbc1_test {

	use super::Mbc1;
	use std::fs;
	use shared::traits::Bus;

	#[test]
	fn test_mbc1_get()  {
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap();
		let mbc = Mbc1::new(rom_file);
		let data = mbc.get(0x00000000);
		assert_eq!(data, 0xc3);
	}

	#[test]
	fn test_mbc1_set_lock() {
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap(); // MBC1 + RAM + BATTERY
		let mut mbc = Mbc1::new(rom_file);

		// assert_eq!(mbc.data[0x149], 0x02);

		// assert_eq!(mbc.ram, vec!(0, 0));

		mbc.set(0x01f5, 0x0a).unwrap();
		assert_eq!(mbc.ram_lock, true);

		mbc.set(0x034b, 0x03).unwrap();
		assert_eq!(mbc.ram_lock, false)
	}

	#[test]
	fn test_mbc1_bank_mod() {
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap(); // MBC1 + RAM + BATTERY
		let mut mbc = Mbc1::new(rom_file);

		mbc.set(0x6abc, 4).unwrap();
		assert_eq!(mbc.bank_mode, false);

		mbc.set(0x7abc, 3).unwrap();
		assert_eq!(mbc.bank_mode, true);
	}

	#[test]
	fn test_mbc1_reg2_0() {
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap(); // MBC1 + RAM + BATTERY
		let mut mbc = Mbc1::new(rom_file);

		mbc.set(0x2156, 0x00).unwrap();
		assert_eq!(mbc.rom_bank, 0x01);
	}

	#[test]
	fn test_mbc1_reg2_1a() {
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap(); // MBC1 + RAM + BATTERY
		let mut mbc = Mbc1::new(rom_file);

		mbc.set(0x2156, 0x1a).unwrap();
		assert_eq!(mbc.rom_bank, 0x1a);
	}

	#[test]
	fn test_mbc1_reg2_14() {
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap(); // MBC1 + RAM + BATTERY
		let mut mbc = Mbc1::new(rom_file);

		mbc.set(0x2fff, 0x14).unwrap();
		assert_eq!(mbc.rom_bank, 0x15);
	}

	#[test]
	fn test_mbc1_reg3_3_14() { // rom_bank previously set to 0x15 so
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap(); // MBC1 + RAM + BATTERY
		let mut mbc = Mbc1::new(rom_file);

		mbc.set(0x4f4f, 0x03).unwrap();
		assert_eq!(mbc.rom_bank, 0x60);
		mbc.set(0x2fff, 0x14).unwrap();
		assert_eq!(mbc.rom_bank, 0x75);
		// mbc.set(0x2fff, 0x14);
		// assert_eq!(mbc.rom_bank, 0x75);

		// mbc.set(0x2fff, 0x42);
		// assert_eq!(mbc.rom_bank, 0x43);
	}

	#[test]
	fn test_mbc1_ram_bank() {

		// LA RAM EST INIT A 0 FAIRE UN TRUC QUI LOAD LA RAM DLA CART A PARTIR DE LA BONNE @
		let rom_file = fs::read("/Users/guvillat/GBMU/roms/Metroid.gb").unwrap(); // MBC1 + RAM + BATTERY
		let mut mbc = Mbc1::new(rom_file);

		mbc.set(0x01f5, 0x0a).unwrap();
		assert_eq!(mbc.ram_lock, true);

		mbc.set(0x7abc, 3).unwrap();// enable bank_mode
		assert_eq!(mbc.bank_mode, true);

		mbc.set(0x4f4f, 0x03).unwrap();
		assert_eq!(mbc.ram_bank, 3);

		mbc.set(0xafcf, 0x99).unwrap();
		// assert_eq!(mbc.ram_bank, 1);

		let data = mbc.get(0x0000afcf);
		assert_eq!(data, 0x99)
	}
}