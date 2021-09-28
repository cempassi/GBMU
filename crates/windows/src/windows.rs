use iced_wgpu::wgpu::Instance;
use iced_winit::winit::event::Event;
use iced_winit::winit::event_loop::EventLoop;
use soc::SOC;

use crate::debugger;
//use crate::emulator;

pub struct Windows {}

impl Windows {
    pub fn run(mut soc: SOC) {
        let event_loop = EventLoop::new();
        let instance = Instance::new(iced_wgpu::wgpu::Backends::PRIMARY);
        let mut debugger = debugger::Debugger::new(&event_loop, &instance, &soc);
        //let mut emulator = emulator::Emulator::new(&event_loop, &instance);

        //let (emulator_id, mut emulator) = emulator::generate_emulator(&event_loop);

        event_loop.run(move |event, _, control_flow| {
            soc.run_once();
            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent { event, window_id } if window_id == debugger.id => {
                    debugger.process_event(event, control_flow);
                }
                // Event::WindowEvent { event, window_id } if window_id == emulator_id => {
                //     emulator::process_event(&mut emulator, event, control_flow);
                // }
                Event::MainEventsCleared => {
                    // If there are events pending
                    if !debugger.state.state.is_queue_empty() {
                        // We update iced
                        debugger.update();

                        // and request a redraw
                        debugger.window.request_redraw();
                    }
                }
                Event::RedrawRequested(window_id) if window_id == debugger.id => {
                    debugger.redraw();
                }
                // Event::RedrawRequested(window_id) if window_id == emulator_id => {
                //     // Do emulator redraw
                // }
                _ => (),
            }
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
