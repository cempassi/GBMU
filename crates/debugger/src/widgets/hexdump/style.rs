//! Style for an [`Hexview`] widget.
//!
//! [`Hexview`]: ../native/hexview/struct.Hexview.html
use iced_native::Color;

/// The apperance of an [`Hexview`].
///
/// [`Hexview`]: ../../native/hexview/struct.Hexview.html
#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Color,
    pub line_color: Color,
    pub offset_color: Color,
    pub data_color: Color,
    pub non_printable_color: Option<Color>,
    pub cursor_color: Color,
}

/// A set of styles for an [`Hexview`]
///
/// [`Hexview`]: ../../native/hexview/struct.Hexview.html
pub trait StyleSheet {
    /// The active style of an [`Hexview`].
    ///
    /// [`Hexview`]: ../../native/hexview/struct.Hexview.html
    fn active(&self) -> Style;
}

/// Built-in light theme for [`Hexview`].
///
/// [`Hexview`]: ../../native/hexview/truct.Hexview.html
#[allow(missing_debug_implementations)]
pub struct Light;

/// Built-in dark theme for [`Hexview`].
///
/// [`Hexview`]: ../../native/hexview/struct.Hexview.html
#[allow(missing_debug_implementations)]
pub struct Dark;

impl Light {
    const ACTIVE_STYLE: Style = Style {
        background_color: Color::from_rgb(1.0, 1.0, 1.0),
        line_color: Color::from_rgb(0.75, 0.75, 0.75),
        offset_color: Color::from_rgb(0.33, 0.33, 0.33),
        data_color: Color::from_rgb(0.196, 0.196, 0.196),
        non_printable_color: Some(Color::from_rgb(0.64, 0.64, 0.64)),
        cursor_color: Color::from_rgb(0.63, 0.63, 0.63),
    };
}

impl Dark {
    const ACTIVE_STYLE: Style = Style {
        background_color: Color::from_rgb(0.18, 0.21, 0.22),
        line_color: Color::from_rgb(0.278, 0.33, 0.345),
        offset_color: Color::from_rgb(0.294, 0.372, 0.372),
        data_color: Color::from_rgb(0.44, 0.53, 0.53),
        non_printable_color: Some(Color::from_rgb(0.27, 0.368, 0.368)),
        cursor_color: Color::from_rgb(0.15, 0.38, 0.44),
    };
}

impl StyleSheet for Light {
    fn active(&self) -> Style {
        Self::ACTIVE_STYLE
    }
}

impl StyleSheet for Dark {
    fn active(&self) -> Style {
        Self::ACTIVE_STYLE
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Light)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
