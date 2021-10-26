#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Position {
    yscroll: u8,
    xscroll: u8,
    ly: u8,
    lycompare: u8,
    ywindow: u8,
    xwindow: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Field {
    Xscroll,
    Yscroll,
    Ly,
    LyCmp,
    Ywindow,
    Xwindow,
}

impl Position {
    pub fn ly_cmp(&self) -> bool {
        self.ly == self.lycompare
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
}
