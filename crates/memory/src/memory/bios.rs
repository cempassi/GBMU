use shared::traits::Bus;

pub type Bios = &[u8; 256] ;

pub const DMG : Bios = include_bytes!("../../../../ressources/bios/dmg_boot.bin");
pub const DMG0 : Bios = include_bytes!("../../../../ressources/bios/dmg0_rom.bin");

impl Bus<usize> for Bios {
    type Item = u8;
    type Result = ();
    type Data = u8;

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
        self[address] = data;
    }

    fn get(&self, address: usize) -> Self::Item {
        self[address]
    }
}

pub enum Type {
    DMG,
    DMG0
}
