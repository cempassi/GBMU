use std::cell::RefCell;
use std::rc::Rc;

pub type Runner = Rc<RefCell<Cycle>>;

const LINE_LENGTH: u32 = 456;

#[derive(Debug, Default)]
pub struct Cycle {
    state: State,
    ticks: u32,
}

impl Cycle {
    pub fn check(&mut self) -> bool {
        self.step();
        match self.state {
            State::Tick => {
                self.state = State::Idle;
                true
            }
            State::Line(ticks) => {
                if ticks < LINE_LENGTH {
                    true
                } else {
                    self.state = State::Idle;
                    false
                }
            }
            _ => false,
        }
    }

    fn step(&mut self) {
        if let State::Line(ref mut ticks) = self.state {
            println!("Processing line, currently at tick {} on 456", *ticks);
            self.ticks += 1;
            *ticks += 1;
        }
    }

    pub fn tick(&mut self) {
        self.state = State::Tick;
    }

    pub fn line(&mut self) {
        println!("Line processing mode!");
        let current_line = self.ticks % LINE_LENGTH;
        println!("Starting at: {}", current_line);
        self.state = State::Line(current_line);
    }
}

#[derive(Debug)]
pub enum State {
    Tick,
    Line(u32),
    Idle,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}
