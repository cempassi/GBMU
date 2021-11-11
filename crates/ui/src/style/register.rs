use super::{Style, Theme};
use iced::{container, Background, Color};

pub struct Register {
    text_color: Option<Color>,
    background: Option<Background>,
    border_radius: f32,
    border_width: f32,
    border_color: Color,
}

const TEXT_LIGHT: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);

const BACKGROUND_LIGHT: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);

const BORDER_LIGHT: Color =
    Color::from_rgb(0x00 as f32 / 255.0, 0x18 as f32 / 255.0, 0x4 as f32 / 255.0);

const TEXT_DARK: Color = Color::from_rgb(
    0x22 as f32 / 255.0,
    0x74 as f32 / 255.0,
    0xA5 as f32 / 255.0,
);

const BACKGROUND_DARK: Color = Color::from_rgb(
    0x22 as f32 / 255.0,
    0x74 as f32 / 255.0,
    0xA5 as f32 / 255.0,
);

const BORDER_DARK: Color = Color::from_rgb(
    0x22 as f32 / 255.0,
    0x74 as f32 / 255.0,
    0xA5 as f32 / 255.0,
);

impl Register {
    fn light() -> Self {
        Self {
            text_color: TEXT_LIGHT.into(),
            background: BACKGROUND_LIGHT.into(),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: BORDER_LIGHT,
        }
    }

    fn dark() -> Self {
        Self {
            text_color: TEXT_DARK.into(),
            background: BACKGROUND_DARK.into(),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: BORDER_DARK,
        }
    }
}

impl container::StyleSheet for Register {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: self.text_color,
            background: self.background,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }
}

impl Style for Register {
    fn style(theme: Theme) -> Self {
        match theme {
            Theme::Light => Register::light(),
            Theme::Dark => Register::dark(),
        }
    }
}
