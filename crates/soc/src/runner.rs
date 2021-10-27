use super::mode::Mode;
use super::processor::Finished;
use std::cell::RefCell;
use std::rc::Rc;

pub type Runner = Rc<RefCell<Cycle>>;

#[derive(Debug, Default)]
pub struct Cycle {
    mode: Mode,
    pub(crate) redraw: bool,
    lines: u32,
    ticks: u32,
    last_cpu: u8,
    last_ppu: u8,
}

impl Cycle {
    pub fn check_redraw(&mut self, status: &mut Vec<Finished>) {
        for status in status {
            match status {
                Finished::Cpu(cycles) => {
                    self.mode.update_processing();
                    self.last_cpu = *cycles;
                    self.redraw = true;
                }
                Finished::Ppu(cycles) => {
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

    pub fn tick(&mut self) {
        println!("Tick processing mode!");
        self.mode = Mode::Tick;
    }

    pub fn line(&mut self) {
        println!("Line processing mode!");
        self.mode = Mode::Line(self.ticks);
        self.ticks = 0;
    }

    pub fn frame(&mut self) {
        println!("Frame processing mode!");
        self.mode = Mode::Line(self.lines);
        self.lines = 0;
    }

    pub fn ppu(&mut self) {
        println!("Ppu cycle processing mode!");
        self.mode = Mode::Ppu;
    }

    pub fn cpu(&mut self) {
        println!("Cpu cycle processing mode!");
        self.mode = Mode::Cpu;
    }
}
