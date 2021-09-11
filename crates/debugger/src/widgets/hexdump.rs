mod consts;
mod renderer;
mod state;
mod style;
mod utils;

use iced_native::{
    event::Status, layout, Element, Event, Font, Hasher, Layout, Length, Point, Rectangle, Size,
    Widget,
};
use std::{hash::Hash, marker::PhantomData};
use utils::clamp;

pub use state::State;

/// A view into a region of bytes.
///
/// The widget owns the bytes it shows, so be careful when using
/// [`State::set_bytes`] with huge data.
///
/// [`State::set_bytes`]: struct.State.html#method.set_bytes
#[allow(missing_debug_implementations)]
pub struct Hexdump<'a, Message, Renderer: renderer::Renderer> {
    state: &'a mut State,
    style: Renderer::Style,
    header_font: Font,
    data_font: Font,
    font_size: f32,
    column_count: u8,
    message: PhantomData<Message>,
}

impl<'a, Message, Renderer: renderer::Renderer> Hexdump<'a, Message, Renderer> {
    /// Creates a new Hexdump.
    pub fn new(state: &'a mut State) -> Self {
        Self {
            state,
            style: Renderer::Style::default(),
            font_size: 17.0,
            header_font: Font::Default,
            data_font: Font::Default,
            column_count: 16,
            message: PhantomData,
        }
    }

    /// Sets the style of an [`Hexdump`].
    ///
    /// [`Hexdump`]: struct.Heview.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the size of the fonts in an [`Hexview`].
    ///
    /// [`Hexview`]: struct.Heview.html
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Sets the font for column headers and offsets in [`Hexview`].
    ///
    /// [`Hexview`]: struct.Heview.html
    pub fn header_font(mut self, font: Font) -> Self {
        self.header_font = font;
        self
    }

    /// Sets the font for bytes and ASCII representation in an [`Hexview`].
    ///
    /// [`Hexview`]: struct.Heview.html
    pub fn data_font(mut self, font: Font) -> Self {
        self.data_font = font;
        self
    }

    /// Sets the amount of columns in an [`Hexview`].
    ///
    /// `count` will be clamped to a number in the range `1..=32`.
    ///
    /// [`Hexview`]: struct.Heview.html
    pub fn column_count(mut self, count: u8) -> Self {
        self.column_count = clamp(count, 1, 32);
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Hexdump<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(Length::Fill);
        let max_width = limits.max().width;
        let rows =
            (self.state.bytes.len() as f32 / self.column_count as usize as usize as f32).ceil();
        let rows_size = (self.font_size + consts::LINE_SPACING) * rows;

        // Vertical margins + top headers + rows
        let height = consts::MARGINS.y * 2.0 + self.font_size + consts::LINE_SPACING + rows_size;

        layout::Node::new(Size::new(max_width, height))
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        _messages: &mut Vec<Message>,
    ) -> Status {
        use iced_native::keyboard::{Event as KeyboardEvent, KeyCode};
        use iced_native::mouse::{Button as MouseButton, Event as MouseEvent};

        let bytes_len = self.state.bytes.len();
        let column_count = self.column_count as usize;
        let cursor = self.state.cursor;
        let keyboard_focus = self.state.keyboard_focus;
        let test_offset = self.state.test_offset;
        let debug_enabled = self.state.debug_enabled;
        let _last_click_pos = self.state.last_click_pos;

        match event {
            Event::Mouse(MouseEvent::ButtonPressed(MouseButton::Left)) => {
                if !layout.bounds().contains(cursor_position) {
                    return Status::Ignored;
                }

                self.state.is_dragging = true;

                let cursor_from_pos = renderer.cursor_offset(
                    layout.bounds(),
                    cursor_position,
                    self.data_font,
                    self.font_size,
                    column_count as usize as usize,
                    false,
                    &self.state.bytes,
                );
                println!("Cursor from pos: {:?}", cursor_from_pos);

                if let Some(cursor) = cursor_from_pos {
                    self.state.cursor = cursor;
                }

                self.state.selection = None;
                self.state.last_click_pos = Some(cursor_position);

                let click = iced_native::mouse::Click::new(cursor_position, self.state.last_click);

                self.state.last_click = Some(click);
            }

            Event::Mouse(MouseEvent::ButtonReleased(MouseButton::Left)) => {
                if let Some(pos) = self.state.last_click_pos.take() {
                    if cursor_position == pos {
                        self.state.selection = None;
                    }
                }

                self.state.is_dragging = false;
                self.state
                    .set_keyboard_focus(layout.bounds().contains(cursor_position));
            }

            Event::Mouse(MouseEvent::CursorMoved { .. }) => {
                if self.state.is_dragging {
                    let cursor_from_pos = renderer.cursor_offset(
                        layout.bounds(),
                        cursor_position,
                        self.data_font,
                        self.font_size,
                        column_count as usize as usize,
                        true,
                        &self.state.bytes,
                    );

                    if let Some(new_cursor) = cursor_from_pos {
                        if new_cursor < cursor {
                            self.state.selection = Some((new_cursor, cursor))
                        } else if new_cursor > cursor {
                            self.state.selection = Some((cursor, new_cursor))
                        } else {
                            self.state.selection = None;
                        }
                    }

                    println!("Selection: {:?}", self.state.selection);
                }
            }

            Event::Keyboard(KeyboardEvent::KeyPressed { key_code, .. }) => {
                let line_start = cursor / column_count as usize as usize * column_count as usize;
                let line_end = (line_start + column_count as usize - 1).min(bytes_len - 1);
                let cursor_guard_left = cursor > 0 && keyboard_focus;
                let cursor_guard_right = if bytes_len > 0 {
                    self.state.cursor < bytes_len - 1 && keyboard_focus
                } else {
                    false
                };
                let cursor_guard_up = cursor >= column_count as usize && keyboard_focus;
                let cursor_guard_down = bytes_len > 0 && keyboard_focus;
                let cursor_guard_home = cursor > line_start && keyboard_focus;
                let cursor_guard_end = cursor < line_end && keyboard_focus;
                let cursor_guard_pageup = cursor > 0 && keyboard_focus;
                let cursor_guard_pagedown = bytes_len > 0 && keyboard_focus;
                let test_offset_guard_left = test_offset > f32::MIN && debug_enabled;
                let test_offset_guard_right = test_offset < f32::MAX && debug_enabled;

                match key_code {
                    // Cursor movement
                    KeyCode::Left if cursor_guard_left => self.state.cursor -= 1,
                    KeyCode::Right if cursor_guard_right => self.state.cursor += 1,
                    KeyCode::Up if cursor_guard_up => self.state.cursor -= column_count as usize,
                    KeyCode::Down if cursor_guard_down => {
                        if cursor + column_count as usize <= bytes_len - 1 {
                            self.state.cursor += column_count as usize;
                        } else {
                            self.state.cursor = bytes_len - 1;
                        }
                    }
                    KeyCode::Home if cursor_guard_home => self.state.cursor = line_start,
                    KeyCode::End if cursor_guard_end => self.state.cursor = line_end,
                    // TODO: Calculate pages based on visible lines
                    KeyCode::PageUp if cursor_guard_pageup => self.state.cursor = 0,
                    KeyCode::PageDown if cursor_guard_pagedown => {
                        if bytes_len > 0 {
                            self.state.cursor = bytes_len - 1;
                        }
                    }

                    // Test offset
                    KeyCode::Minus if test_offset_guard_left => self.state.test_offset -= 0.01,
                    KeyCode::Equals if test_offset_guard_right => self.state.test_offset += 0.01,

                    // Debug
                    KeyCode::D if keyboard_focus => self.state.debug_enabled = !debug_enabled,

                    _ => (),
                }
            }

            _ => (),
        }
        Status::Captured
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            layout.bounds(),
            cursor_position,
            &self.style,
            self.font_size,
            self.column_count as usize,
            self.state.keyboard_focus,
            self.state.cursor,
            self.state.test_offset,
            self.state.debug_enabled,
            self.header_font,
            self.data_font,
            &self.state.selection,
            &self.state.bytes,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;

        std::any::TypeId::of::<Marker>().hash(state);
        self.state.bytes_hash.hash(state);
    }
}

impl<'a, Message, Renderer> From<Hexdump<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Message: 'a,
{
    fn from(hexview: Hexdump<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(hexview)
    }
}
