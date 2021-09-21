use iced_wgpu::{checkbox, container, Color};

pub struct Container;

const BACKGROUND: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);

const CHECKED: Color = Color::from_rgb(
    0x28 as f32 / 255.0,
    0x3F as f32 / 255.0,
    0x27 as f32 / 255.0,
);

// const HOVERED: Color = Color::from_rgb(
//     0x67 as f32 / 255.0,
//     0x7B as f32 / 255.0,
//     0xC4 as f32 / 255.0,
// );

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(0xFF, 0xFF, 0xFF).into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background:  BACKGROUND.into(),
            checkmark_color: CHECKED,
            border_radius: 10.0,
            border_width: 2.0,
            border_color: Color::BLACK,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            ..self.active(is_checked)
        }
    }
}
