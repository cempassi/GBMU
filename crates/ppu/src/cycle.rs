#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cycle {
    Ppu(u32),
    Finished,
}
