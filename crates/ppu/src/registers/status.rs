use super::Control;
use modular_bitfield::{bitfield, specifiers::B2, BitfieldSpecifier};
// /// Bit     Name                                    Usage notes
// /// 6       LYC=LY STAT Interrupt source            (1=Enable) (Read/Write)
// /// 5       Mode 2 OAM STAT Interrupt source        (1=Enable) (Read/Write)
// /// 4       Mode 1 VBlank STAT Interrupt source     (1=Enable) (Read/Write)
// /// 3       Mode 0 HBlank STAT Interrupt source     (1=Enable) (Read/Write)
// /// 2       LYC=LY Flag                             (0=Different, 1=Equal) (Read Only)
// /// 1-0     Mode Flag                               (Mode 0-3, see below) (Read Only)
// ///             0: HBlank
// ///             1: VBlank
// ///             2: Searching OAM
// ///             3: Transferring Data to LCD Controller

#[bitfield]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Status {
    #[bits = 2]
    pub(crate) mode: Mode,
    pub lyc_ly: bool,
    hblank_interupt: bool,
    vblank_interupt: bool,
    oam_interupt: bool,
    lyc_ly_interupt: bool,
    #[skip]
    unused: bool,
}

#[derive(BitfieldSpecifier, Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Hblank,
    Vblank,
    Oam,
    Transfert,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Hblank
    }
}

impl Status {
    pub fn get_mode(&self) -> Mode {
        self.mode()
    }
}
