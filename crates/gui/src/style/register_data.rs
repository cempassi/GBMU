use super::{Theme, Style};
use iced_wgpu::{container, Color};
use iced_winit::Background;

pub struct RegisterData {
    text_color: Option<Color>,
    background: Option<Background>,
    border_radius: f32,
    border_width: f32,
    border_color: Color,
}

const REGISTER: Color = Color::from_rgb(
    0x22 as f32 / 255.0,
    0x74 as f32 / 255.0,
    0xA5 as f32 / 255.0,
);

impl RegisterData {
    fn light() -> Self {
        Self {
            text_color: REGISTER.into(),
            background: REGISTER.into(),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: REGISTER,
        }
    }

    fn dark() -> Self {
        Self {
            text_color: REGISTER.into(),
            background: REGISTER.into(),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: REGISTER,
        }
    }
}

impl container::StyleSheet for RegisterData {
    fn style(&self) -> container::Style {
        container::Style{
            text_color: self.text_color,
            background: self.background,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }
}

impl Style for RegisterData {
    fn theme(theme: Theme) -> Self {
        match theme {
            Theme::Light => RegisterData::light(),
            Theme::Dark => RegisterData::dark(),
        }
    }
}
