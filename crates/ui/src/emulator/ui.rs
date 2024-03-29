use iced::{Command, Element};
use iced_wgpu::Renderer;
use iced_wgpu::Text;
use iced_winit::Program;

#[derive(Default)]
pub struct UserInterface {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Program for UserInterface {
    type Renderer = Renderer;

    type Message = Message;

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello Mate").size(20).into()
    }
}
