use enum_iterator::IntoEnumIterator;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;

const MAP_ROW_LEN: u16 = 32;
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Coordinates {
    yscroll: u8,
    xscroll: u8,
    ly: u8,
    lycompare: u8,
    ywindow: u8,
    xwindow: u8,
}

// The structure that holds the wrapped iterator and the current state.
pub struct XRange {
    count: u8,
    current: u8,
}

impl Iterator for XRange {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0 => None,
            _ => {
                let item = self.current;
                self.count -= 1;
                self.current = self.current.wrapping_add(1);
                Some(item)
            }
        }
    }
}

impl XRange {
    pub fn new(current: u8) -> Self {
        Self { count: 22, current }
    }
}

#[repr(u16)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive, IntoEnumIterator, PartialEq, Eq, Clone, Copy)]
pub enum Field {
    Ly = 0xFF44,
    LyCmp = 0xFF45,
    Yscroll = 0xFF42,
    Xscroll = 0xFF43,
    Ywindow = 0xFF4A,
    Xwindow = 0xFF4B,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Field::Xscroll => write!(f, "X Scroll"),
            Field::Yscroll => write!(f, "Y Scroll"),
            Field::Ly => write!(f, "Ly"),
            Field::LyCmp => write!(f, "Ly Compare"),
            Field::Ywindow => write!(f, "Y Window"),
            Field::Xwindow => write!(f, "X Window"),
        }
    }
}

impl Coordinates {
    pub fn ly_compare(&self) -> bool {
        self.ly == self.lycompare
    }

    pub fn update(&self, dst: &mut Self) {
        *dst = Self { ..*self };
    }

    pub fn x_range(&self) -> XRange {
        XRange::new(self.xscroll)
    }

    pub fn y(&self) -> usize {
        self.ly.wrapping_add(self.yscroll) as usize
    }

    pub fn tile_line(&self) -> usize {
        (self.y() % 8) as usize
    }

    pub fn map_row_offset(&self) -> u16 {
        (self.y() as u16 / 8) * MAP_ROW_LEN
    }

    pub fn offset(&self, x: usize) -> usize {
        self.ly as usize * crate::ppu::FRAME_WIDTH + x
    }

    pub fn get(&self, field: Field) -> u8 {
        match field {
            Field::Xscroll => self.xscroll,
            Field::Yscroll => self.yscroll,
            Field::Ly => self.ly,
            Field::LyCmp => self.lycompare,
            Field::Ywindow => self.ywindow,
            Field::Xwindow => self.xwindow,
        }
    }

    pub fn set(&mut self, field: Field, data: u8) {
        match field {
            Field::Xscroll => self.xscroll = data,
            Field::Yscroll => self.yscroll = data,
            Field::Ly => self.ly = data,
            Field::LyCmp => self.lycompare = data,
            Field::Ywindow => self.ywindow = data,
            Field::Xwindow => self.xwindow = data,
        }
    }

    pub fn increase(&mut self, field: Field) {
        match field {
            Field::Xscroll => self.xscroll += 1,
            Field::Yscroll => self.yscroll += 1,
            Field::Ly => self.ly += 1,
            Field::LyCmp => self.lycompare += 1,
            Field::Ywindow => self.ywindow += 1,
            Field::Xwindow => self.xwindow += 1,
        }
    }

    pub fn is_equal(&mut self, operation: Field, value: u8) -> bool {
        match operation {
            Field::Xscroll => self.xscroll == value,
            Field::Yscroll => self.yscroll == value,
            Field::Ly => self.ly == value,
            Field::LyCmp => self.lycompare == value,
            Field::Ywindow => self.ywindow == value,
            Field::Xwindow => self.xwindow == value,
        }
    }

    pub fn is_lower(&mut self, operation: Field, value: u8) -> bool {
        match operation {
            Field::Xscroll => self.xscroll < value,
            Field::Yscroll => self.yscroll < value,
            Field::Ly => self.ly < value,
            Field::LyCmp => self.lycompare < value,
            Field::Ywindow => self.ywindow < value,
            Field::Xwindow => self.xwindow < value,
        }
    }

    pub fn clear(&mut self, operation: Field) {
        match operation {
            Field::Xscroll => self.xscroll = 0,
            Field::Yscroll => self.yscroll = 0,
            Field::Ly => self.ly = 0,
            Field::LyCmp => self.lycompare = 0,
            Field::Ywindow => self.ywindow = 0,
            Field::Xwindow => self.xwindow = 0,
        }
    }

    /// Get a reference to the coordinates's xscroll.
    pub fn xscroll(&self) -> u8 {
        self.xscroll
    }
}
