use std::cell::RefCell;
use std::rc::Rc;

pub type Runner = Rc<RefCell<Cycle>>;

/// Number of ticks in a line
const LINE_LENGTH: u32 = 456;

/// Number of Lines in a frame
const FRAME_LENGTH: u32 = 144;

#[derive(Debug, Default)]
pub struct Cycle {
    state: State,
    redraw: bool,
    lines: u32,
    ticks: u32,
}

impl Cycle {
    pub fn check_tick(&mut self) -> bool {
        self.step();
        !matches!(self.state, State::Idle)
    }

    pub fn check_redraw(&mut self, status: bool) -> bool {
        match self.state {
            State::Tick => {
                self.state = State::Idle;
                status
            }
            State::Line(ticks) if ticks == LINE_LENGTH => {
                self.state = State::Idle;
                true
            }
            State::Frame(lines) if lines == FRAME_LENGTH => {
                self.state = State::Idle;
                true
            }
            _ => false,
        }
    }

    fn step(&mut self) {
        if let State::Line(ticks) = self.state {
            println!("Processing line, currently at tick {} on 456", ticks);
            if self.state.increase() {
                self.lines += 1;
                self.ticks = 0;
            }
        } else if let State::Frame(lines) = self.state {
            println!("Processing frame, currently at line {} on 120", lines);
            if self.state.increase() {
                self.lines = 0;
            }
        } else {
            self.ticks += 1;
            if self.ticks == LINE_LENGTH {
                self.lines += 1;
                self.ticks = 0;
            }
            if self.lines == FRAME_LENGTH {
                self.lines = 0;
                self.ticks = 0;
            }
        }
    }

    pub fn tick(&mut self) {
        self.state = State::Tick;
    }

    pub fn line(&mut self) {
        println!("Line processing mode!");
        self.state = State::Line(self.ticks);
        self.ticks = 0;
    }

    pub fn frame(&mut self) {
        println!("Line processing mode!");
        self.state = State::Line(self.lines);
        self.lines = 0;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Tick,
    Line(u32),
    Frame(u32),
    Idle,
}

impl State {
    pub fn increase(&mut self) -> bool {
        match self {
            State::Line(ref mut ticks) => {
                *ticks += 1;
                *ticks == LINE_LENGTH
            }
            State::Frame(ref mut lines) => {
                *lines += 1;
                *lines == FRAME_LENGTH
            }
            _ => false,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}
