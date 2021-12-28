use super::mode::Mode;
use shared::{Finished, Redraw};
use std::time::Instant;

/// The status struct controls the different execution modes of the soc.
/// The code here is really critical.
#[derive(Debug, Default)]
pub struct System {
    cpu: cpu::Cpu,
    mode: Mode,
    pub redraw: Redraw,
    lines: u32,
    ticks: u32,
    last_cpu_cycle: u8,
    last_line_cycle: u16,
    frames: u64,
}

impl System {
    pub fn new(cpu: cpu::Cpu) -> Self {
        Self {
            cpu,
            ..Self::default()
        }
    }
    //  The order of priority of the matches really matters.
    //  As a general guideline, priority is as follows:
    //      - Stop execution if an error occurred
    //      - Finished::Frame must redraw no matter what.
    //      - Mode::Tick must go idle whatever what else is finished
    //      - Stop execution if button condition met
    //      - Update info if something Finished (Line or Cpu)
    //      - Do nothing
    //
    //
    pub fn check_redraw(&mut self, status: Vec<Finished>) {
        for status in status {
            match (status, self.mode) {
                (Finished::Error(_error), _) => {
                    self.mode.idle();
                    self.redraw.update(Redraw::All);
                    //println!("[SOC] Error in check redraw: {}", error);
                }
                (Finished::Frame, Mode::Tick) => {
                    self.frames += 1;
                    self.mode.idle();
                    self.reset_count();
                    //println!("[SOC] Finished a frame {} (tick)", self.frames);
                    self.redraw.update(Redraw::All);
                }
                (Finished::Frame, Mode::Second(_)) => {
                    self.frames += 1;
                    //println!("[SOC] Finished frame {} (seconds)", self.frames);
                    self.redraw.update(Redraw::Emulator);
                }
                (Finished::Frame, Mode::Run(_)) => {
                    self.frames += 1;
                    //println!("[SOC] Finished frame {} (seconds)", self.frames);
                    self.redraw.update(Redraw::All);
                }
                (Finished::Frame, Mode::Frame) => {
                    self.mode.idle();
                    self.reset_count();
                    self.frames += 1;
                    //println!("[SOC] Finished frame {} (frame)", self.frames);
                    self.redraw.update(Redraw::All);
                }
                (Finished::Frame, _) => {
                    self.reset_count();
                    self.frames += 1;
                    //println!("[SOC] Finished frame {} (frame)", self.frames);
                    //println!("[SOC] Reseting frame count");
                    self.redraw.update(Redraw::All);
                }
                (_, Mode::Tick) => {
                    //println!("[SOC] Tick finished");
                    self.mode.idle();
                    self.redraw.update(Redraw::Debugger);
                }
                (_, Mode::Second(time)) if time.elapsed().as_secs() > 1 => {
                    // println!(
                    //     "[SOC] Timed mode finished in {} sec",
                    //     time.elapsed().as_secs()
                    // );
                    self.frames = 0;
                    self.mode.idle();
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Cpu(cycles), Mode::Instruction) => {
                    //println!("[SOC] Instruction finished");
                    self.mode.idle();
                    self.last_cpu_cycle = cycles;
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Line(cycles), Mode::Line) => {
                    //println!("[SOC] Line finished");
                    self.mode.idle();
                    self.add_line();
                    self.last_line_cycle = cycles;
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Cpu(cycles), _) => {
                    self.last_cpu_cycle = cycles;
                }
                (Finished::Line(cycles), _) => {
                    self.add_line();
                    self.last_line_cycle = cycles;
                }
                _ => {}
            }
        }
    }

    // !! Carefull !!: This function is extremely important as it decides
    // if the soc should stop making progress. Carefull logic is needed.
    pub fn processing(&mut self) -> bool {
        !(self.is_breakpoint()
            || self.mode.second() && self.redraw.ready()
            || self.mode.run() && self.redraw.ready()
            || self.is_idle())
    }

    pub fn step(&mut self) {
        self.ticks += 1;
    }

    fn is_breakpoint(&mut self) -> bool {
        if let Mode::Breakpoint(breakpoint) = self.mode {
            let pc = self.cpu.borrow().registers.pc;
            if pc == breakpoint {
                self.mode = Mode::Idle;
                self.redraw.update(Redraw::Debugger);
                println!("[SOC] Breakpoint Reached");
                true
            } else {
                false
            }
        } else {
            false
        }
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

    pub fn mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn second(&mut self) {
        self.mode = Mode::Second(Instant::now());
    }

    pub fn run(&mut self) {
        self.mode = Mode::Run(Instant::now());
    }
}

#[cfg(test)]
mod test_status {
    use super::Finished;
    use super::Instant;
    use super::Mode;
    use super::Redraw;
    use super::System;

    #[test]
    fn test_status_after_tick() {
        let mut status = System {
            mode: Mode::Tick,
            ..System::default()
        };

        let finished = vec![Finished::Nope, Finished::Nope];

        status.check_redraw(finished);
        assert_eq!(status.redraw, Redraw::Debugger);
        assert_eq!(status.mode, Mode::Idle);
    }

    #[test]
    fn test_processing_after_tick_returns_false() {
        let mut status = System {
            mode: Mode::Tick,
            ..System::default()
        };

        let finished = vec![Finished::Nope, Finished::Nope];
        status.check_redraw(finished);

        assert!(!status.processing());
    }

    #[test]
    fn test_frame_ready_in_second_stops_processing() {
        let mut status = System {
            mode: Mode::Second(Instant::now()),
            redraw: Redraw::Emulator,
            ..System::default()
        };

        assert!(!status.processing());
    }
}
