/// Number of ticks in a line
const LINE_LENGTH: u32 = 456;

/// Number of Lines in a frame
const FRAME_LENGTH: u32 = 144;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Tick,
    Line(u32),
    Frame(u32),
    Idle,
    Cpu,
    Ppu,
}

impl Mode {
    pub fn increase(&mut self) -> bool {
        match self {
            Self::Line(ref mut ticks) => {
                *ticks += 1;
                *ticks == LINE_LENGTH
            }
            Self::Frame(ref mut lines) => {
                *lines += 1;
                *lines == FRAME_LENGTH
            }
            _ => false,
        }
    }

    pub fn update_processing(&mut self) {
        if *self == Mode::Cpu || *self == Mode::Ppu {
            *self = Mode::Idle;
        }
    }

    pub fn is_processing(&self) -> bool {
        matches!(self, Self::Cpu | Self::Ppu)
    }

    pub fn check_redraw(&mut self) -> bool {
        match self {
            Self::Tick => {
                *self = Self::Idle;
                true
            }
            Self::Line(ticks) if *ticks == LINE_LENGTH => {
                *self = Self::Idle;
                true
            }
            Self::Frame(lines) if *lines == FRAME_LENGTH => {
                *self = Self::Idle;
                true
            }
            _ => false,
        }
    }

    /// Check if we reached end of line
    pub fn is_eol(num: u32) -> bool {
        num == LINE_LENGTH
    }

    /// Check if we reached end of frame
    pub fn is_eof(num: u32) -> bool {
        num == FRAME_LENGTH
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Idle
    }
}
