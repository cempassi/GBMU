use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Destination {
    Jap = 0x00,
    Other = 0x01,
}
