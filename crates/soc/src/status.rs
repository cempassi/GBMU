use super::mode::Mode;
use shared::Finished;

#[derive(Debug, Default)]
pub struct Status {
    mode: Mode,
    pub(crate) redraw: bool,
    lines: u32,
    ticks: u32,
    last_cpu: u8,
    last_ppu: u8,
}

impl Status {
    pub fn check_redraw(&mut self, status: &mut Vec<Finished>) {
        for status in status {
            match status {
                Finished::Cpu(cycles) if self.mode == Mode::Cpu => {
                    self.mode.update_processing();
                    self.last_cpu = *cycles;
                    self.redraw = true;
                }
                Finished::Line(cycles) if self.mode == Mode::Ppu => {
                    self.mode.update_processing();
                    self.last_ppu = *cycles;
                    self.redraw = true;
                }
                Finished::Frame(cycles) if self.mode == Mode::Ppu => {
                    self.mode.update_processing();
                    self.last_ppu = *cycles;
                    self.redraw = true;
                }
                Finished::Error(_) => self.redraw = true,
                Finished::Nope if self.mode.is_processing() => (),
                Finished::Nope if !self.redraw => self.redraw = self.mode.check_redraw(),
                _ => (),
            }
        }
    }

    pub fn step(&mut self) {
        if let Mode::Line(ticks) = self.mode {
            println!("Processing line, currently at tick {} on 456", ticks);
            if self.mode.increase() {
                self.lines += 1;
                self.ticks = 0;
            }
        } else if let Mode::Frame(lines) = self.mode {
            println!("Processing frame, currently at line {} on 120", lines);
            if self.mode.increase() {
                self.lines = 0;
            }
        } else {
            self.ticks += 1;
            if Mode::is_eol(self.ticks) {
                self.lines += 1;
                self.ticks = 0;
            }
            if Mode::is_eof(self.ticks) {
                self.lines = 0;
                self.ticks = 0;
            }
        }
    }

    pub fn is_idle(&self) -> bool {
        self.mode == Mode::Idle
    }

    pub fn tick(&mut self) {
        self.mode = Mode::Tick;
    }

    pub fn line(&mut self) {
        self.mode = Mode::Line(self.ticks);
        self.ticks = 0;
    }

    pub fn frame(&mut self) {
        self.mode = Mode::Line(self.lines);
        self.lines = 0;
    }

    pub fn ppu(&mut self) {
        self.mode = Mode::Ppu;
    }

    pub fn cpu(&mut self) {
        self.mode = Mode::Cpu;
    }
}
