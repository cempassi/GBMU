use crate::Error;
use std::fmt;
use std::{future::Future, pin::Pin};

pub type Output = Pin<Box<dyn Future<Output = Result<Finished, Error>>>>;

#[derive(Debug)]
pub enum Finished {
    Cpu(u8),
    Line(u16),
    Frame,
    Error(Error),
    Nope,
}

impl fmt::Display for Finished {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Finished::Cpu(ticks) => write!(f, "Finished instruction. ticks: {}", *ticks),
            Finished::Line(ticks) => write!(f, "Finished line. ticks: {}", *ticks),
            Finished::Frame => write!(f, "Finished Frame."),
            Finished::Error(ticks) => write!(f, "Error: {}", *ticks),
            Finished::Nope => write!(f, "Processing"),
        }
    }
}

pub enum Process {
    Cpu,
    Ppu,
}

impl Finished {
    pub fn finish(result: Result<Self, Error>) -> Self {
        //println!("{:?}", result);
        if let Err(error) = result {
            Self::Error(error)
        } else {
            result.unwrap()
        }
    }
}

pub trait Run {
    fn run(self) -> Output;
}
