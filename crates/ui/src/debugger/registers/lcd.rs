//use enum_iterator::IntoEnumIterator;
use super::PpuMsg;
use crate::debugger::widgets::Register;
use iced_wgpu::{Column, Renderer, Row};
use iced_winit::Element;
use ppu::Coordinates;
use ppu::Field;

pub struct Lcd {
    coordinates: Coordinates,
    builder: Register,
}

impl Lcd {
    pub fn new(ppu: &ppu::Ppu) -> Self {
        let builder = Register::new(20, 7, 8);
        let mut coordinates = Coordinates::default();
        ppu.borrow().reload_coordinates(&mut coordinates);
        Self {
            coordinates,
            builder,
        }
    }

    pub fn update(&mut self, ppu: &ppu::Ppu) {
        ppu.borrow().reload_coordinates(&mut self.coordinates);
    }

    pub fn make(&self, field: Field) -> Row<PpuMsg> {
        let data = self.coordinates.get(field).to_string();
        let name = format!("{}", field);
        self.builder.render(name, data)
    }

    pub fn pair(&self, field_a: Field, field_b: Field) -> Row<PpuMsg> {
        let field_a = self.make(field_a);
        let field_b = self.make(field_b);
        Row::new().push(field_a).push(field_b)
    }

    pub fn view(&self) -> Element<PpuMsg, Renderer> {
        let pair1 = self.pair(Field::Ly, Field::LyCmp);
        let pair2 = self.pair(Field::Yscroll, Field::Xscroll);
        let pair3 = self.pair(Field::Ywindow, Field::Xwindow);

        Column::new().push(pair1).push(pair2).push(pair3).into()
    }
}
