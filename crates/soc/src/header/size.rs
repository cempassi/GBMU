use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Rom {
    KByte32 = 0x00,
    KByte64 = 0x01,
    KByte128 = 0x02,
    KByte256 = 0x03,
    KByte512 = 0x04,
    MByte1 = 0x05,
    MByte2 = 0x06,
    MByte4 = 0x07,
    MByte8 = 0x08,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Ram {
    #[num_enum(alternatives = [0x01])]
    NoRam = 0x00,
    KByte8 = 0x02,
    KByte32 = 0x03,
    KByte128 = 0x04,
    KByte64 = 0x05,
}
