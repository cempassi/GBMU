#![allow(dead_code, unused_attributes, unused_imports)]
pub use super::Registers;

pub use super::{Coordinates, Field};

impl Registers {
    fn check_ly(&mut self, field: Field) {
        if matches!(field, Field::Ly | Field::LyCmp) {
            self.status.set_lyc_ly(self.coordinates.ly_cmp());
        }
    }

    pub fn increase(&mut self, field: Field) {
        self.coordinates.increase(field);
        self.check_ly(field);
    }

    pub fn clear(&mut self, field: Field) {
        self.coordinates.clear(field);
        self.check_ly(field);
    }

    pub fn is_equal(&mut self, field: Field, data: u8) -> bool {
        self.check_ly(field);
        self.coordinates.is_equal(field, data)
    }

    pub fn is_lower(&mut self, field: Field, data: u8) -> bool {
        self.check_ly(field);
        self.coordinates.is_lower(field, data)
    }
}
