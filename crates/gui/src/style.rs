pub mod theme;
mod register;
mod memory;

pub use theme::Theme;
pub use register::Register;

pub trait Style {
    fn style(theme: Theme) -> Self;
}
