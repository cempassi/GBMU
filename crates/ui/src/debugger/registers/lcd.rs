//use enum_iterator::IntoEnumIterator;
use super::PpuMsg;
use crate::debugger::widgets::Register;
use iced_wgpu::{Column, Renderer, Row};
use iced_winit::Element;
use ppu::Field;

pub struct Lcd {
    lcd: ppu::Lcd,
    builder: Register,
}

impl Lcd {
    pub fn new(src: &ppu::Lcd) -> Self {
        let lcd = Clone::clone(&*src);
        let builder = Register::new(20, 5, 8);
        Self { lcd, builder }
    }

    pub fn update(&mut self, lcd: &ppu::Lcd) {
        lcd.set(&mut self.lcd)
    }

    pub fn make(&self, field: Field) -> Row<PpuMsg> {
        let data = self.lcd.get_lcd(field).to_string();
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

        Column::new().push(pair1).into()
    }
}
