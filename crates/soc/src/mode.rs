use std::time::Instant;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Tick,
    Instruction,
    Line,
    Frame,
    Idle,
    Second(Instant),
}

impl Mode {
    pub fn idle(&mut self) {
        *self = Self::Idle;
    }

    pub fn second(&self) -> bool {
       matches!(self, Self::Second(_))
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Idle
    }
}
