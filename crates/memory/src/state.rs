#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Bios,
    Rom
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cycle {
    Cpu(u8),
    Finished,
}
