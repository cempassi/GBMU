mod position;
pub use position::{Field, Position};
// use modular_bitfield::{bitfield, specifiers::B2};

// /// LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
// /// Bit	 Name                           Usage notes
// /// 7	 LCD and PPU enable             0=Off, 1=On
// /// 6	 Window tile map area           0=9800-9BFF, 1=9C00-9FFF
// /// 5	 Window enable                  0=Off, 1=On
// /// 4	 BG and Window tile data area   0=8800-97FF, 1=8000-8FFF
// /// 3	 BG tile map area               0=9800-9BFF, 1=9C00-9FFF
// /// 2	 OBJ size                       0=8x8, 1=8x16
// /// 1	 OBJ enable                     0=Off, 1=On
// /// 0	 BG and Window enable/priority	0=Off, 1=On
//
// #[bitfield]
// #[derive(Debug, Default, Copy, Clone, PartialEq)]
// #[allow(dead_code)]
// pub struct Control {
//     priority: bool,
//     obj_enabled: bool,
//     obj_size: bool,
//     bg_area: bool,
//     tile_data: bool,
//     window_enabled: bool,
//     window_area: bool,
//     enabled: bool,
// }

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
//
// #[bitfield]
// #[allow(dead_code)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Status {
    //     mode: B2,
    pub lyc_ly: bool,
    //     hblank_interupt: bool,
    //     vblank_interupt: bool,
    //     oam_interupt: bool,
    //     lyc_ly_interupt: bool,
    //     #[skip]
    //     unused: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Lcd {
    // //control: Control,
    status: Status,
    position: Position,
}

impl Lcd {
    fn check_ly(&mut self, field: Field) {
        if matches!(field, Field::Ly | Field::LyCmp) {
            self.status.lyc_ly = self.position.ly_cmp();
        }
    }

    pub fn set(&self, dst: &mut Self) {
        dst.status = self.status;
        dst.position = self.position;
    }

    pub fn increase(&mut self, field: Field) {
        self.position.increase(field);
        self.check_ly(field);
    }

    pub fn clear(&mut self, field: Field) {
        self.position.clear(field);
        self.check_ly(field);
    }

    pub fn is_equal(&mut self, field: Field, data: u8) -> bool {
        self.check_ly(field);
        self.position.is_equal(field, data)
    }

    pub fn is_lower(&mut self, field: Field, data: u8) -> bool {
        self.check_ly(field);
        self.position.is_lower(field, data)
    }
}
