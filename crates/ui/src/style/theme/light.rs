use iced_wgpu::{button, checkbox, container, Color};
use iced_winit::Vector;

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

const CONTAINER_BACKGROUND: Color = Color::from_rgb(
    0xF7 as f32 / 255.0,
    0xF5 as f32 / 255.0,
    0xDE as f32 / 255.0,
);

const CONTAINER_TEXT: Color = Color::from_rgb(
    0x1C as f32 / 255.0,
    0x1C as f32 / 255.0,
    0x1C as f32 / 255.0,
);

const CONTAINER_BORDER: Color = Color::from_rgb(
    0x22 as f32 / 255.0,
    0x74 as f32 / 255.0,
    0xA5 as f32 / 255.0,
);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: CONTAINER_TEXT.into(),
            background: CONTAINER_BACKGROUND.into(),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: CONTAINER_BORDER,
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: BACKGROUND.into(),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: Color::BLACK,
            shadow_offset: Vector::new(0.1, 0.1),
            text_color: Color::BLACK,
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: BACKGROUND.into(),
            checkmark_color: CHECKED,
            border_radius: 10.0,
            border_width: 5.0,
            border_color: Color::BLACK,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            ..self.active(is_checked)
        }
    }
}
