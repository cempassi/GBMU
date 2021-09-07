use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Cgb {
    RetroCompatible = 0x80,
    CgbOnly = 0xC0,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Sgb {
    Unsupported = 0x00,
    Supported = 0x03,
}
