use iced_glutin::glutin::{event::Event, event_loop::EventLoop};

use crate::debugger;
use crate::emulator;

pub struct Windows {}

impl Windows {
    pub fn run() {
        let event_loop = EventLoop::new();
        let mut debugger = debugger::Debugger::new(&event_loop);

        let (emulator_id, mut emulator) = emulator::generate_emulator(&event_loop);

        event_loop.run(move |event, _, control_flow| match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, window_id } if window_id == debugger.id => {
                *control_flow = debugger.process_event(event)
            }
            Event::WindowEvent { event, window_id } if window_id == emulator_id => {
                *control_flow = emulator::process_event(&mut emulator, event)
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !debugger.state.state.is_queue_empty() {
                    // We update iced
                    debugger.update();

                    // and request a redraw
                    debugger.context.window().request_redraw();
                }
            }
            Event::RedrawRequested(window_id) if window_id == debugger.id => {
                debugger.redraw();
                debugger.context.swap_buffers().unwrap();
            }
            Event::RedrawRequested(window_id) if window_id == emulator_id => {
                // Do emulator redraw
            }
            _ => (),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
