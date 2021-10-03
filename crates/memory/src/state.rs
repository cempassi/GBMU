#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rom {
    Bios,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cycle {
    Cpu(u8),
    Finished,
}
