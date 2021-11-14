use super::consts;

#[derive(Debug, PartialEq, Eq)]
pub enum Area {
    Bios,
    Rom,
    Vram,
    _ExtRam,
    Wram,
    EchoRam,
    IOReg,
    Hram,
}

impl Area {
    pub fn relative(self, address: u16) -> usize {
        let result = match self {
            Area::Bios => address,
            Area::Rom => address,
            Area::Vram => address - consts::VRAM_MIN,
            Area::_ExtRam => address - consts::EXT_RAM_MIN,
            Area::Wram => address - consts::WRAM_MIN,
            Area::EchoRam => address - consts::ECHO_MIN,
            Area::IOReg => address - consts::IOREG_MIN,
            Area::Hram => address - consts::HRAM_MIN,
        };
        result as usize
    }
}
