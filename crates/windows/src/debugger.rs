use glium::glutin::event_loop::EventLoop;
use glium::glutin::ContextBuilder;
use glium::Display;
use winit::window::{WindowBuilder, WindowId};

pub fn generate_debugger(event_loop: &EventLoop<()>) -> (WindowId, Display) {
    let title = "Debugger";
    let window_builder = WindowBuilder::new().with_title(title);
    let context_builder = ContextBuilder::new();
    let display = Display::new(window_builder, context_builder, event_loop).unwrap();
    let window_id = display.gl_window().window().id();
    display.gl_window().window().set_visible(false);
    (window_id, display)
}
