mod memory;
mod register;
pub mod theme;

pub use register::Register;
pub use theme::Theme;

pub trait Style {
    fn style(theme: Theme) -> Self;
}
