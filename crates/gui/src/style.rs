pub mod theme;
mod register_data;
mod memory;

pub use theme::Theme;
pub use register_data::Register;

pub trait Style {
    fn style(theme: Theme) -> Self;
}
