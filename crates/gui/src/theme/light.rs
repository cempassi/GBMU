use iced_glow::{container, Color};

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(0xFF, 0xFF, 0xFF).into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}
