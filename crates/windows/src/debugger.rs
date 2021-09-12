use iced_glutin::glutin::window::CursorIcon;
use iced_glutin::glutin::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};

use iced_glutin::mouse;

pub struct Debugger {
    pub id: WindowId,
    pub context: ContextWrapper<PossiblyCurrent, Window>,
    pub state: gui::Debugger,
    pub gl: glow::Context,
}

impl Debugger {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let title = "Debugger";
        let window_builder = WindowBuilder::new().with_title(title);
        let windowed_context = ContextBuilder::new()
            .build_windowed(window_builder, event_loop)
            .unwrap();

        let (windowed_context, gl) = {
            unsafe {
                let windowed_context = windowed_context.make_current().unwrap();

                let gl = glow::Context::from_loader_function(|s| {
                    windowed_context.get_proc_address(s) as *const _
                });

                (windowed_context, gl)
            }
        };
        let state = gui::Debugger::new(windowed_context.window(), &gl);

        let id = windowed_context.window().id();
        Self {
            id,
            context: windowed_context,
            state,
            gl,
        }
    }

    pub fn process_event(&self, event: WindowEvent) -> ControlFlow {
        match event {
            WindowEvent::Resized(_physical_size) => ControlFlow::Wait,
            WindowEvent::CloseRequested => ControlFlow::Wait,
            _ => ControlFlow::Wait,
        }
    }
    pub fn update(&mut self) {
        self.state.update(self.context.window().scale_factor());
    }

    pub fn redraw(&mut self) {
        let mouse = self.state.draw(&self.gl);

        self.context
            .window()
            .set_cursor_icon(mouse_interaction(mouse));
    }
}

pub fn mouse_interaction(interaction: mouse::Interaction) -> CursorIcon {
    use mouse::Interaction;

    match interaction {
        Interaction::Idle => CursorIcon::Default,
        Interaction::Pointer => CursorIcon::Hand,
        Interaction::Working => CursorIcon::Progress,
        Interaction::Grab => CursorIcon::Grab,
        Interaction::Grabbing => CursorIcon::Grabbing,
        Interaction::Crosshair => CursorIcon::Crosshair,
        Interaction::Text => CursorIcon::Text,
        Interaction::ResizingHorizontally => CursorIcon::EwResize,
        Interaction::ResizingVertically => CursorIcon::NsResize,
    }
}
