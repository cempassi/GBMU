#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Tick,
    Instruction,
    Line,
    Frame,
    Idle,
}

impl Mode {
    pub fn idle(&mut self) {
        *self = Self::Idle;
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Idle
    }
}
