use super::View;
use iced::{Alignment, Column, Element, Row};

use super::PpuMsg;
use crate::{
    debugger::widgets::{Register, Text},
    style::Theme,
};

impl View<PpuMsg> for ppu::ppu::LcdControl {
    fn view(&self, _theme: Theme) -> Element<PpuMsg> {
        let title = Text::new("Control").medium_it(20);
        let control = Column::new().align_items(Alignment::Center).push(title);

        let lcd_enabled = Register::render("lcd_enabled".to_string(), self.window_enabled.to_string());
        let line = Row::new().push(lcd_enabled);
        let control = control.push(line);

        let window_enabled = Register::render(
            "window_enabled".to_string(),
            self.window_enabled.to_string(),
        );
        // let window_area = format!("{:#06X}", self.window_map_start);
        // let window_area = Register::render("window_area".to_string(), window_area);
        let line = Row::new().push(window_enabled);//.push(window_area);
        let control = control.push(line);

        // let bg_area = format!("{:#06X}", self.bg_area);
        // let bg_area = Register::render("bg_area".to_string(), bg_area);
        // let data_area = format!("{:#06X}", self.data_area);
        // let data_area = Register::render("data_area".to_string(), data_area);
        // let line = Row::new().push(bg_area).push(data_area);
        //let control = control.push(line);

        let sprite_enabled = Register::render(
            "sprite_enabled".to_string(),
            self.sprites_enabled.to_string(),
        );
        //let sprite_size = format!("{:#06X}", self.sprite_size);
        //let sprite_size = Register::render("sprite_size".to_string(), sprite_size);
        let line = Row::new().push(sprite_enabled);//.push(sprite_size);
        let control = control.push(line);

        control.into()
    }
}
