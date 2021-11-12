use super::mode::Mode;
use shared::{Finished, Redraw};
use std::time::Instant;

/// The status struct controls the different execution modes of the soc.
/// The code here is really critical.
#[derive(Debug, Default)]
pub struct Status {
    mode: Mode,
    pub redraw: Redraw,
    lines: u32,
    ticks: u32,
    last_cpu_cycle: u8,
    last_line_cycle: u16,
}

impl Status {
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
                (Finished::Error(_), _) => {
                    self.mode.idle();
                    self.redraw.update(Redraw::All);
                }
                (Finished::Frame, Mode::Tick) => {
                    println!("[SOC] Finished a frame (tick)");
                    self.mode.idle();
                    self.reset_count();
                    self.redraw.update(Redraw::All);
                }
                (Finished::Frame, Mode::Second(_)) => {
                    println!("[SOC] Finished a frame in (seconds)");
                    self.redraw.update(Redraw::Emulator);
                }
                (Finished::Frame, Mode::Frame) => {
                    println!("[SOC] Finished a frame (frame)");
                    self.mode.idle();
                    self.reset_count();
                    self.redraw.update(Redraw::All);
                }
                (Finished::Frame, _) => {
                    self.reset_count();
                    self.redraw.update(Redraw::All);
                }
                (_, Mode::Tick) => {
                    self.mode.idle();
                    self.redraw.update(Redraw::Debugger);
                }
                (_, Mode::Second(time)) if time.elapsed().as_secs() > 1 => {
                    self.mode.idle();
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Cpu(cycles), Mode::Instruction) => {
                    self.mode.idle();
                    self.last_cpu_cycle = cycles;
                    self.redraw.update(Redraw::Debugger);
                }
                (Finished::Line(cycles), Mode::Line) => {
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
                _ => (),
            }
        }
    }

    // !! Carefull !!: This function is extremely important as it decides
    // if the soc should stop making progress. Carefull logic is needed.
    pub fn processing(&self) -> bool {
        !(self.mode.second() && self.redraw.ready() || self.is_idle())
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

    pub fn mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn second(&mut self) {
        self.mode = Mode::Second(Instant::now());
    }

    /// Get the redraw status.
    pub fn redraw(&self) -> Redraw {
        self.redraw
    }
}

#[cfg(test)]
mod test_status {
    use super::Finished;
    use super::Instant;
    use super::Mode;
    use super::Redraw;
    use super::Status;

    #[test]
    fn test_status_after_tick() {
        let mut status = Status {
            mode: Mode::Tick,
            ..Status::default()
        };

        let finished = vec![Finished::Nope, Finished::Nope];

        status.check_redraw(finished);
        assert_eq!(status.redraw, Redraw::Debugger);
        assert_eq!(status.mode, Mode::Idle);
    }

    #[test]
    fn test_processing_after_tick_returns_false() {
        let mut status = Status {
            mode: Mode::Tick,
            ..Status::default()
        };

        let finished = vec![Finished::Nope, Finished::Nope];
        status.check_redraw(finished);

        assert!(!status.processing());
    }

    #[test]
    fn test_frame_ready_in_second_stops_processing() {
        let status = Status {
            mode: Mode::Second(Instant::now()),
            redraw: Redraw::Emulator,
            ..Status::default()
        };

        assert!(!status.processing());
    }
}
