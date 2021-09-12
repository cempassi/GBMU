use iced_glow::{container, Color};
pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}
