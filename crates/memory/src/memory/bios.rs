pub type Bios<'a> = &'a [u8; 256];

pub const DMG: Bios = include_bytes!("../../../../ressources/bios/dmg_boot.bin");