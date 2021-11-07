#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Redraw {
    Nope,
    All,
    Debugger,
    Emulator,
}

impl Default for Redraw {
    fn default() -> Self {
        Self::Nope
    }
}

impl Redraw {
    pub fn update(&mut self, status: Self) {
        *self = match (&self, status) {
            (Redraw::Nope, Redraw::Nope) => Redraw::Nope,
            (Redraw::Nope, Redraw::All) => Redraw::All,
            (Redraw::Nope, Redraw::Debugger) => Redraw::Debugger,
            (Redraw::Nope, Redraw::Emulator) => Redraw::Emulator,
            (Redraw::All, Redraw::Nope) => Redraw::All,
            (Redraw::All, Redraw::All) => Redraw::All,
            (Redraw::All, Redraw::Debugger) => Redraw::All,
            (Redraw::All, Redraw::Emulator) => Redraw::All,
            (Redraw::Debugger, Redraw::Nope) => Redraw::Debugger,
            (Redraw::Debugger, Redraw::All) => Redraw::All,
            (Redraw::Debugger, Redraw::Debugger) => Redraw::Debugger,
            (Redraw::Debugger, Redraw::Emulator) => Redraw::All,
            (Redraw::Emulator, Redraw::Nope) => Redraw::Emulator,
            (Redraw::Emulator, Redraw::All) => Redraw::All,
            (Redraw::Emulator, Redraw::Debugger) => Redraw::All,
            (Redraw::Emulator, Redraw::Emulator) => Redraw::Emulator,
        };
    }
}
