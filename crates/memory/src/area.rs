use super::consts;

#[derive(Debug, PartialEq, Eq)]
pub enum Area {
    Bios,
    Rom,
    _Vram,
    _ExtRam,
    Wram,
    _EchoRam,
    _Oam,
    _IOReg,
    _HighRam,
    // Interrupt,
}

impl Area {
    pub fn relative(self, address: u16) -> usize {
        let result = match self {
            Area::Bios => address,
            Area::Rom => address,
            Area::_Vram => address - consts::ROM_MIN,
            Area::_ExtRam => address - consts::EXT_RAM_MIN,
            Area::Wram => address - consts::WRAM_MIN,
            Area::_EchoRam => address - consts::ECHO_RAM_MIN,
            Area::_Oam => address - consts::OAM_MIN,
            Area::_IOReg => address - consts::IOREG_MIN,
            Area::_HighRam => address - consts::HIGH_MIN,
            // Area::Interrupt => consts::INTERUPT_ENABLE,
        };
        result as usize
    }
}
