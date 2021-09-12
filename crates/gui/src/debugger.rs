mod components;

use components::register;
use iced_glow::{Backend, Renderer, Settings, Viewport};
use iced_glutin::{glutin::window::Window, mouse::Interaction, Clipboard, Debug, Point, Size};
use iced_native::{program, Column, Command, Element, Program, Text};
use winit::dpi::PhysicalPosition;

pub struct Debugger {
    registers: register::Registers,
}

pub struct State {
    pub state: program::State<Debugger>,
    pub clipboard: Clipboard,
    pub debug: Debug,
    pub viewport: Viewport,
    pub renderer: Renderer,
    pub cursor: PhysicalPosition<f64>,
}

impl State {
    pub fn new(window: &Window, context: &glow::Context) -> Self {
        let debugger = Debugger::new();
        let mut debug = Debug::new();
        let clipboard = Clipboard::connect(window);

        let physical_size = window.inner_size();
        let size = Size::new(physical_size.width, physical_size.height);
        let viewport = Viewport::with_physical_size(size, window.scale_factor());

        let cursor = PhysicalPosition::new(-1.0, -1.0);
        let logical_cursor = cursor.to_logical(window.scale_factor());
        let point = Point::new(logical_cursor.x, logical_cursor.y);

        let mut renderer = Renderer::new(Backend::new(context, Settings::default()));

        let state = program::State::new(
            debugger,
            viewport.logical_size(),
            point,
            &mut renderer,
            &mut debug,
        );

        Self {
            state,
            clipboard,
            debug,
            cursor,
            viewport,
            renderer,
        }
    }

    pub fn update(&mut self, scale_factor: f64) {
        let logical_cursor = self.cursor.to_logical(scale_factor);
        let point = Point::new(logical_cursor.x, logical_cursor.y);

        let _ = self.state.update(
            self.viewport.logical_size(),
            point,
            &mut self.renderer,
            &mut self.clipboard,
            &mut self.debug,
        );
    }

    pub fn draw(&mut self, context: &glow::Context) -> Interaction {
        self.renderer.backend_mut().draw(
            context,
            &self.viewport,
            self.state.primitive(),
            &self.debug.overlay(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ForRegister(register::Message),
}

impl Program for Debugger {
    type Clipboard = Clipboard;
    type Message = Message;
    type Renderer = Renderer;

    fn update(
        &mut self,
        message: Message,
        _clipboard: &mut Self::Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ForRegister(message) => {
                self.registers.update(message);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message, Self::Renderer> {
        let column = Column::new()
            .push(Text::new("Hello, world! Are we doing this or what?").color([0.0, 0.0, 1.0]));

        Element::new(column)
    }
}

impl Debugger {
    fn new() -> Self {
        Self {
            registers: register::Registers::default(),
        }
    }

    // fn title(&self) -> String {
    //     String::from("Hello World")
    // }
}
