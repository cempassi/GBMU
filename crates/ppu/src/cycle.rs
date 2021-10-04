#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cycle {
    Ppu(u8),
    Finished,
}
