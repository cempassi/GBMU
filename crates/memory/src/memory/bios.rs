pub type Bios<'a> = &'a [u8; 256];

pub const DMG: Bios = include_bytes!("../../../../ressources/bios/dmg_boot.bin");
//pub const DMG0: Bios = include_bytes!("../../../../ressources/bios/dmg0_rom.bin");

// pub enum Type {
//     DMG,
//     DMG0,
// }
