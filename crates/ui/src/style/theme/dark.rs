use iced::{button, checkbox, container, Color, Vector};

const BACKGROUND: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);

const SURFACE: Color = Color::from_rgb(
    0x40 as f32 / 255.0,
    0x44 as f32 / 255.0,
    0x4B as f32 / 255.0,
);

// const ACCENT: Color = Color::from_rgb(
//     0x6F as f32 / 255.0,
//     0xFF as f32 / 255.0,
//     0xE9 as f32 / 255.0,
// );

const ACTIVE: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x89 as f32 / 255.0,
    0xDA as f32 / 255.0,
);

// const HOVERED: Color = Color::from_rgb(
//     0x67 as f32 / 255.0,
//     0x7B as f32 / 255.0,
//     0xC4 as f32 / 255.0,
// );

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

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: BACKGROUND.into(),
            border_radius: 10.0,
            border_width: 5.0,
            border_color: Color::BLACK,
            shadow_offset: Vector::new(0.5, 0.5),
            text_color: Color::BLACK,
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: if is_checked { ACTIVE } else { SURFACE }.into(),
            checkmark_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color {
                a: 0.8,
                ..if is_checked { ACTIVE } else { SURFACE }
            }
            .into(),
            ..self.active(is_checked)
        }
    }
}
