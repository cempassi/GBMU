use super::MemoryMsg;
use super::View;
use crate::debugger::widgets::memory::Hexdump;
use iced_native::Element;
use memory::Bus;

impl View<MemoryMsg> for Bus {
    fn view(self, _theme: crate::style::Theme) -> Element<'static, MemoryMsg, iced_wgpu::Renderer> {
        let rc = self.borrow();
        let vector = rc.as_ref().as_ref();
        Hexdump::render(vector).into()
    }
}
