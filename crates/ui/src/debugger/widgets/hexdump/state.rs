use iced_winit::Point;
use memory::Bus;
use std::hash::Hasher;

/// state of hexdump
/// The local state of an [`Hexdump`].

#[derive(Debug)]
pub struct State {
    pub data: Bus,
    pub cursor: usize,
    pub bytes_hash: u64,
    pub keyboard_focus: bool,
    pub test_offset: f32,
    pub debug_enabled: bool,
    pub last_click: Option<iced_winit::mouse::click::Click>,
    pub last_click_pos: Option<Point>,
    pub selection: Option<(usize, usize)>,
    pub is_dragging: bool,
    pub mouse_pos: Point,
}

impl State {
    /// Sets the data [`Hexdump`] will be working with.
    ///
    /// Currently, we just clone the data into a Vec, which should work fine for
    /// small amounts of data.
    pub fn new(bus: Bus) -> Self {
        let mut hasher = iced_winit::Hasher::default();
        hasher.write(bus.borrow().as_ref().as_ref());
        let data = bus;
        let cursor = 0;
        let bytes_hash = hasher.finish();
        let selection = None;
        let keyboard_focus = false;
        let test_offset = 0.0;
        let debug_enabled = false;
        let last_click = None;
        let last_click_pos = None;
        let is_dragging = false;
        let mouse_pos = Point::default();

        Self {
            data,
            cursor,
            bytes_hash,
            keyboard_focus,
            test_offset,
            debug_enabled,
            last_click,
            last_click_pos,
            selection,
            is_dragging,
            mouse_pos,
        }
    }

    /// Sets the keyboard focus of an [`Hexdump`].
    ///
    /// The keyboard focus is automatically determined by whether the user has
    /// cicked inside the widget, but can be manually set in order to use
    /// shortcuts and move around.
    ///
    /// [`Hexdump`]: struct.Heview.html
    pub fn set_keyboard_focus(&mut self, focus: bool) {
        self.keyboard_focus = focus;
    }
}
