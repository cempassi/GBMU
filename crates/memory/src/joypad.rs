use shared::{Interrupt, Interrupts};

const SELECT: u8 = 0x30;
const SELECT_ACTION: u8 = 0x10;
const SELECT_DIRECTION: u8 = 0x20;

#[derive(Debug)]
pub struct Joypad {
    actions: u8,
    directions: u8,
    data: u8,
    pub interrupt: Interrupts,
}

#[derive(Copy, Clone)]
pub enum JoypadKey {
    Right = 0x1,
    Left = 0x2,
    Up = 0x4,
    Down = 0x8,
    A = 0x10,
    B = 0x20,
    Select = 0x40,
    Start = 0x80,
}

impl JoypadKey {
    pub fn shift(&self) -> u8 {
        (*self as u8) >> 4
    }
}

impl Joypad {
    pub fn new(interrupt: Interrupts) -> Joypad {
        Self {
            actions: 0x0F,
            directions: 0x0F,
            data: 0xFF,
            interrupt,
        }
    }

    pub fn get(&self) -> u8 {
        self.data
    }

    pub fn set(&mut self, value: u8) {
        self.data = (self.data & 0xCF) | (value & SELECT);
        self.update();
    }

    fn update(&mut self) {
        let old_values = self.data & 0xF;
        let mut new_values = 0xF;

        if self.data & SELECT_ACTION == 0x00 {
            new_values &= self.actions;
        }
        if self.data & SELECT_DIRECTION == 0x00 {
            new_values &= self.directions;
        }

        if old_values == 0xF && new_values != 0xF {
            self.interrupt.borrow_mut().request(Interrupt::Joypad);
        }

        self.data = (self.data & 0xF0) | new_values;
    }

    pub fn keydown(&mut self, key: JoypadKey) {
        match key {
            JoypadKey::Right => self.actions &= !(key as u8),
            JoypadKey::Left => self.actions &= !(key as u8),
            JoypadKey::Up => self.actions &= !(key as u8),
            JoypadKey::Down => self.actions &= !(key as u8),
            JoypadKey::A => self.directions &= !(key.shift()),
            JoypadKey::B => self.directions &= !(key.shift()),
            JoypadKey::Select => self.directions &= !(key.shift()),
            JoypadKey::Start => self.directions &= !(key.shift()),
        }
        self.update();
    }

    pub fn keyup(&mut self, key: JoypadKey) {
        match key {
            JoypadKey::Right => self.actions |= key as u8,
            JoypadKey::Left => self.actions |= key as u8,
            JoypadKey::Up => self.actions |= key as u8,
            JoypadKey::Down => self.actions |= key as u8,
            JoypadKey::A => self.directions |= key.shift(),
            JoypadKey::B => self.directions |= key.shift(),
            JoypadKey::Select => self.directions |= key.shift(),
            JoypadKey::Start => self.directions |= key.shift(),
        }
        self.update();
    }
}
