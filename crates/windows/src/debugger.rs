use gui::debugger;
use iced_glutin::glutin::window::CursorIcon;
use iced_glutin::glutin::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
    ContextBuilder, ContextCurrentState, ContextWrapper, PossiblyCurrent,
};
use iced_glutin::mouse;

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

pub fn generate_debugger(
    event_loop: &EventLoop<()>,
) -> (
    WindowId,
    ContextWrapper<PossiblyCurrent, Window>,
    debugger::State,
    glow::Context,
) {
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
    let debugger = debugger::State::new(windowed_context.window(), &gl);

    let window_id = windowed_context.window().id();
    (window_id, windowed_context, debugger, gl)
}

pub fn process_event<T: ContextCurrentState>(
    _context: &mut ContextWrapper<T, Window>,
    event: WindowEvent,
) -> ControlFlow {
    match event {
        WindowEvent::Resized(_physical_size) => ControlFlow::Wait,
        WindowEvent::CloseRequested => ControlFlow::Wait,
        _ => ControlFlow::Wait,
    }
}

pub fn redraw(debugger: &mut debugger::State, gl: &glow::Context, window: &Window) {
    let mouse = debugger.draw(gl);

    window.set_cursor_icon(mouse_interaction(mouse));
}
