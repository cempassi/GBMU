use super::mode::Mode;
use shared::{Finished, Redraw};
use std::time::Instant;

#[derive(Debug, Default)]
pub struct Status {
    mode: Mode,
    pub redraw: Redraw,
    lines: u32,
    ticks: u32,
    last_cpu_cycle: u8,
    last_line_cycle: u8,
    last_frame_cycle: u8,
}

impl Status {
    pub fn check_redraw(&mut self, status: &mut Vec<Finished>) {
        self.redraw = Redraw::Nope;
        for status in status {
            match (status, self.mode) {
                (_, Mode::Second(time)) if time.elapsed().as_secs() > 1 => {
                    self.mode.idle();
                    self.redraw.update(Redraw::All);
                }
                (Finished::Cpu(cycles), Mode::Instruction) => {
                    self.mode.idle();
                    self.last_cpu_cycle = *cycles;
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Line(cycles), Mode::Line) => {
                    self.mode.idle();
                    self.add_line();
                    self.last_line_cycle = *cycles;
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Frame(cycles), Mode::Frame) => {
                    self.mode.idle();
                    self.reset_count();
                    self.last_frame_cycle = *cycles;
                    self.redraw.update(Redraw::All);
                }
                (Finished::Cpu(cycles), _) => {
                    self.last_cpu_cycle = *cycles;
                }
                (Finished::Line(cycles), _) => {
                    self.add_line();
                    self.last_line_cycle = *cycles;
                }
                (Finished::Frame(cycles), _) => {
                    self.reset_count();
                    self.last_frame_cycle = *cycles;
                    self.redraw.update(Redraw::All);
                }
                (_, Mode::Tick) => {
                    self.mode.idle();
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Error(_), _) => {
                    self.redraw.update(Redraw::All);
                }
                _ => (),
            }
        }
    }

    pub fn step(&mut self) {
        self.ticks += 1;
    }

    pub fn add_line(&mut self) {
        self.lines += 1;
        self.ticks = 0;
    }

    pub fn reset_count(&mut self) {
        self.lines = 0;
        self.ticks = 0;
    }

    pub fn is_idle(&self) -> bool {
        self.mode == Mode::Idle
    }

    pub fn tick(&mut self) {
        self.mode = Mode::Tick;
    }

    pub fn line(&mut self) {
        self.mode = Mode::Line;
    }

    pub fn frame(&mut self) {
        self.mode = Mode::Frame;
    }

    pub fn instruction(&mut self) {
        self.mode = Mode::Instruction;
    }

    pub fn second(&mut self) {
        self.mode = Mode::Second(Instant::now());
    }
}
