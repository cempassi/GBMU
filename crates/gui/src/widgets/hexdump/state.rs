use iced_winit::Point;


/// state of hexdump
/// The local state of an [`Hexdump`].

#[derive(Debug, Default)]
pub struct State {
    pub bytes: Vec<u8>,
    pub cursor: usize,
    pub bytes_hash: u64,
    pub keyboard_focus: bool,
    pub test_offset: f32,
    pub debug_enabled: bool,
    pub selection: Option<(usize, usize)>,
    pub last_click: Option<iced_winit::mouse::click::Click>,
    pub last_click_pos: Option<Point>,
    pub is_dragging: bool,
    pub mouse_pos: Point,
}

impl State {
    /// Sets the data [`Hexdump`] will be working with.
    ///
    /// Currently, we just clone the data into a Vec, which should work fine for
    /// small amounts of data.
    pub fn load(&mut self, bytes: &[u8]) {
        use std::hash::Hasher;

        let mut hasher = iced_winit::Hasher::default();
        hasher.write(bytes);
        self.bytes_hash = hasher.finish();
        self.bytes = bytes.to_vec();
        self.cursor = 0;
        self.selection = None;
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
