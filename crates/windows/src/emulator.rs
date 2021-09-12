use iced_glutin::glutin::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
    ContextBuilder, ContextCurrentState, ContextWrapper, NotCurrent,
};

pub fn generate_emulator(
    event_loop: &EventLoop<()>,
) -> (WindowId, ContextWrapper<NotCurrent, Window>) {
    let title = "GBMU";
    let window_builder = WindowBuilder::new().with_title(title);
    let windowed_context = ContextBuilder::new()
        .build_windowed(window_builder, event_loop)
        .unwrap();
    let window_id = windowed_context.window().id();
    (window_id, windowed_context)
}

pub fn process_event<T: ContextCurrentState>(
    _context: &mut ContextWrapper<T, Window>,
    event: WindowEvent,
) -> ControlFlow {
    match event {
        WindowEvent::Resized(_physical_size) => {
            ControlFlow::Wait
            //
        }
        WindowEvent::CloseRequested => ControlFlow::Exit,
        _ => ControlFlow::Wait,
    }
}
