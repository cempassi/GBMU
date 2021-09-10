use debugger::Debugger;
use iced::{Sandbox, Settings};

pub fn main() -> Result<(), iced::Error> {
    Debugger::run(Settings::default())
}
